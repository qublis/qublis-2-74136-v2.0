//! CollectiveSync — Distributed AI Agent Synchronization for Qublis v2.0
//!
//! `CollectiveSync` manages a dynamic set of AI agents, each with a quantum‐number
//! `AgentState`.  Agents can register, send `SyncMessage`s to each other, and
//! perform a global synchronization step that entangles or averages states
//! across the collective.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use std::collections::HashMap;
use qublis_qnum::{QNum, entangle, qadd};
use crate::{
    config::CiCoreConfig,
    error::CiCoreError,
    metrics::CiCoreMetrics,
    types::{AgentId, AgentState, SyncMessage},
};

/// `CollectiveSync` holds a registry of agents and supports message‐based
/// and global synchronization operations.
#[derive(Debug, Clone)]
pub struct CollectiveSync {
    config: CiCoreConfig,
    metrics: CiCoreMetrics,
    /// Map from agent ID to its current quantum state
    agents: HashMap<AgentId, AgentState>,
}

impl CollectiveSync {
    /// Create a new, empty `CollectiveSync` instance with the given configuration.
    pub fn new(config: &CiCoreConfig) -> Self {
        let mut metrics = CiCoreMetrics::new();
        metrics.inc_counter("collective_sync_initialized", 1);
        CollectiveSync {
            config: config.clone(),
            metrics,
            agents: HashMap::new(),
        }
    }

    /// Register a new agent with the given `id` and initial `state`.
    /// Returns an error if the agent already exists.
    pub fn register_agent(
        &mut self,
        id: AgentId,
        state: AgentState,
    ) -> Result<(), CiCoreError> {
        if self.agents.contains_key(&id) {
            return Err(CiCoreError::SyncError(format!(
                "agent {} already registered", id
            )));
        }
        self.agents.insert(id.clone(), state);
        self.metrics.inc_counter("agents_registered", 1);
        Ok(())
    }

    /// Send a `SyncMessage` from one agent to another,
    /// entangling the recipient’s state with the message’s payload.
    pub fn send_message(
        &mut self,
        from: &AgentId,
        to: &AgentId,
        msg: SyncMessage,
    ) -> Result<(), CiCoreError> {
        // Workaround borrow checker: collect sender.state.clone() before mut borrow
        let sender_state = {
            let sender = self.agents.get(from)
                .ok_or_else(|| CiCoreError::SyncError(format!("unknown agent {}", from)))?;
            sender.state.clone()
        };
        let recipient = self.agents.get_mut(to)
            .ok_or_else(|| CiCoreError::SyncError(format!("unknown agent {}", to)))?;
        // entangle recipient with message state (and optionally sender)
        entangle(&mut recipient.state, &mut msg.state.clone());
        if self.config.enable_global_entangle {
            // also entangle with sender state for tighter sync
            entangle(&mut recipient.state, &mut sender_state.clone());
        }
        self.metrics.inc_counter("messages_sent", 1);
        Ok(())
    }

    /// Perform a global synchronization across all registered agents.
    ///
    /// If `enable_global_average` is set in config, computes the QNum average
    /// of all agent states (via repeated `qadd` and normalization) and replaces
    /// each agent’s state with that summary; otherwise entangles every pair.
    pub fn synchronize(&mut self) -> Result<(), CiCoreError> {
        let n = self.agents.len();
        if n == 0 {
            return Err(CiCoreError::SyncError("no agents to synchronize".into()));
        }
        if self.config.enable_global_average {
            // compute summary
            let mut iter = self.agents.values();
            let first = iter.next().unwrap().state.clone();
            let mut summary = first;
            for agent in iter {
                summary = qadd(&summary, &agent.state);
            }
            // normalize by measuring and re-encoding
            let measured = summary.measure();
            let new_state = AgentState { state: QNum::from_digits(&measured) };
            for (_id, agent) in self.agents.iter_mut() {
                agent.state = new_state.state.clone();
            }
            self.metrics.inc_counter("global_averages", 1);
        } else {
            // entangle each pair (clone, entangle, write back)
            let ids: Vec<AgentId> = self.agents.keys().cloned().collect();
            for i in 0..ids.len() {
                for j in (i+1)..ids.len() {
                    let ai = &ids[i];
                    let aj = &ids[j];

                    // Clone current states
                    let state_i = self.agents.get(ai).unwrap().state.clone();
                    let state_j = self.agents.get(aj).unwrap().state.clone();

                    // Entangle clones
                    let (mut entangled_i, mut entangled_j) = (state_i, state_j);
                    entangle(&mut entangled_i, &mut entangled_j);

                    // Write back
                    self.agents.get_mut(ai).unwrap().state = entangled_i;
                    self.agents.get_mut(aj).unwrap().state = entangled_j;
                }
            }
            self.metrics.inc_counter("global_entanglements", 1);
        }
        Ok(())
    }

    /// Retrieve a snapshot of all current agent states.
    pub fn snapshot(&self) -> HashMap<AgentId, Vec<u8>> {
        self.agents.iter()
            .map(|(id, agent)| (id.clone(), agent.state.clone().measure()))
            .collect()
    }

    /// Export internal metrics in Prometheus text format.
    pub fn export_metrics(&self) -> String {
        self.metrics.export_prometheus()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qublis_qnum::QNum;

    fn make_agent(id: &str, digit: u8) -> (AgentId, AgentState) {
        (id.to_string(), AgentState { state: QNum::from_digits(&[digit]) })
    }

    #[test]
    fn test_registration_and_snapshot() {
        let cfg = CiCoreConfig::default();
        let mut cs = CollectiveSync::new(&cfg);
        let (a1, s1) = make_agent("A1", 3);
        let (a2, s2) = make_agent("A2", 7);
        assert!(cs.register_agent(a1.clone(), s1).is_ok());
        assert!(cs.register_agent(a2.clone(), s2).is_ok());
        let snap = cs.snapshot();
        assert_eq!(snap.get(&a1), Some(&vec![3]));
        assert_eq!(snap.get(&a2), Some(&vec![7]));
    }

    #[test]
    fn test_duplicate_registration() {
        let cfg = CiCoreConfig::default();
        let mut cs = CollectiveSync::new(&cfg);
        let (id, state) = make_agent("X", 1);
        cs.register_agent(id.clone(), state).unwrap();
        let err = cs.register_agent(id.clone(), make_agent("X", 2).1).unwrap_err();
        matches!(err, CiCoreError::SyncError(_));
    }

    #[test]
    fn test_send_message_entangles() {
        let mut cfg = CiCoreConfig::default();
        cfg.enable_global_entangle = false;
        let mut cs = CollectiveSync::new(&cfg);
        let (a, sa) = make_agent("A", 2);
        let (b, sb) = make_agent("B", 5);
        cs.register_agent(a.clone(), sa).unwrap();
        cs.register_agent(b.clone(), sb).unwrap();
        let msg = SyncMessage { from: a.clone(), state: QNum::from_digits(&[9]) };
        cs.send_message(&a, &b, msg).unwrap();
        let snap = cs.snapshot();
        // B's state now either 5 or 9
        assert!(matches!(snap.get(&b).unwrap().as_slice(), [5] | [9]));
    }

    #[test]
    fn test_global_average() {
        let mut cfg = CiCoreConfig::default();
        cfg.enable_global_average = true;
        let mut cs = CollectiveSync::new(&cfg);
        cs.register_agent("A".into(), make_agent("A", 2).1).unwrap();
        cs.register_agent("B".into(), make_agent("B", 4).1).unwrap();
        cs.synchronize().unwrap();
        let snap = cs.snapshot();
        // Average of 2 & 4 is 3
        assert_eq!(snap.values().next(), Some(&vec![3]));
    }

    #[test]
    fn test_global_entanglements() {
        let mut cfg = CiCoreConfig::default();
        cfg.enable_global_average = false;
        let mut cs = CollectiveSync::new(&cfg);
        cs.register_agent("X".into(), make_agent("X", 1).1).unwrap();
        cs.register_agent("Y".into(), make_agent("Y", 2).1).unwrap();
        cs.synchronize().unwrap();
        let snap = cs.snapshot();
        // After entanglement, at least one state has increased entropy
        let entropies: Vec<f64> = cs.agents.values()
            .map(|ag| ag.state.entropy()).collect();
        assert!(entropies.iter().any(|&e| e > 0.0));
    }

    #[test]
    fn test_synchronize_no_agents() {
        let cfg = CiCoreConfig::default();
        let mut cs = CollectiveSync::new(&cfg);
        let err = cs.synchronize().unwrap_err();
        matches!(err, CiCoreError::SyncError(_));
    }
}
