````markdown
# Consensus NeuroFlux Integration

This document describes the **Consensus NeuroFlux** module for **Qublis v2.0** (2-74136), which embeds the NeuroFlux reinforcement-learning agent directly into the QMesh entropic DAG consensus engine. By continuously tuning consensus parameters in real time, NeuroFlux ensures optimal throughput, latency, and stability across dynamic network conditions.

---

## Table of Contents

1. [Purpose](#purpose)  
2. [Design Overview](#design-overview)  
3. [Algorithm](#algorithm)  
4. [Runtime Module: `consensus_neuroflux.rs`](#runtime-module-consensus_neurofluxrs)  
5. [Configuration](#configuration)  
6. [Metrics & Monitoring](#metrics--monitoring)  
7. [Testing](#testing)  
8. [Usage Example](#usage-example)  

---

## Purpose

The Consensus NeuroFlux module dynamically adjusts key consensus parameters—such as tip-selection weights, entropy thresholds for finality, and retro-reorg windows—based on live feedback from the network. It treats the consensus engine as an environment, and NeuroFlux as an agent that observes state, selects actions, and receives rewards.

---

## Design Overview

- **Integration Point**: Hooked into the QMesh runtime loop, invoked once per block‐production epoch.  
- **State Representation**: A `QNum` vector encoding metrics like current tip count, average block latency, cognitive entropy, and validator performance.  
- **Action Space**: Adjustments to consensus parameters (e.g., `entropy_finality`, `max_tips`, `retro_window_secs`).  
- **Reward Function**: Composite metric combining normalized TPS, inverse latency, and stability bonus for low fork rates.  
- **Learning Loop**: On each epoch:
  1. Observe current consensus state.  
  2. Select new parameter tweaks.  
  3. Apply parameters to consensus engine.  
  4. Measure resulting network performance.  
  5. Compute reward and update NeuroFlux policy.  

---

## Algorithm

1. **Observe**:  
   ```text
   s_t = [
     QNum::from_digits(&[tip_count]),
     QNum::from_digits(&[floor(avg_latency_ms)]),
     QNum::from_digits(&[floor(cognitive_entropy)]),
     QNum::from_digits(&[validator_uptime_score])
   ]
````

2. **Select Action** (ε-greedy):

   * With probability ε, explore random parameter set.
   * Otherwise, use policy network π(s\_t) to propose Δparams.

3. **Apply**:

   ```rust
   consensus_config.entropy_finality = clamp(
     consensus_config.entropy_finality + action.delta_entropy,
     min_entropy, max_entropy
   );
   consensus_config.max_tips = (consensus_config.max_tips as isize
     + action.delta_tips).clamp(min_tips, max_tips) as usize;
   ```

4. **Reward**:

   $$
     r_t = w_{\mathrm{TPS}} \cdot \frac{\text{measured\_tps}}{\text{target\_tps}}
         - w_{\mathrm{lat}} \cdot \frac{\text{avg\_latency\_ms}}{\text{max\_latency\_ms}}
         - w_{\mathrm{forks}} \cdot \text{fork\_rate}
   $$

5. **Learn**: Update policy and value networks via TD-learning, optionally refining QBLang reward contracts.

---

## Runtime Module: `consensus_neuroflux.rs`

Below is the complete source for the Consensus NeuroFlux integration in `runtime/consensus_neuroflux.rs`:

```rust
//! Consensus NeuroFlux integration for Qublis v2.0
//!
//! Embeds a NeuroFlux RL agent into the QMesh consensus loop,
//! enabling dynamic tuning of consensus parameters.

use qublis_qnum::QNum;
use crate::config::ConsensusConfig;
use crate::runtime::ConsensusEngine;
use crate::ci_core::NeuroFluxAgent;
use crate::metrics::ConsensusMetrics;

#[derive(Debug)]
pub struct ConsensusNeuroFlux {
    agent: NeuroFluxAgent,
    metrics: ConsensusMetrics,
}

impl ConsensusNeuroFlux {
    /// Initialize NeuroFlux in the consensus engine.
    pub fn new(cfg: &ConsensusConfig) -> Self {
        let mut agent = NeuroFluxAgent::new(&cfg.neuroflux);
        let mut metrics = ConsensusMetrics::new();
        metrics.inc_counter("neuroflux_initialized", 1);
        ConsensusNeuroFlux { agent, metrics }
    }

    /// Called each epoch to observe, act, and learn.
    pub fn tick(&mut self, engine: &mut ConsensusEngine) {
        // 1. Observe current state
        let s = self.collect_state(engine);

        // 2. Select an action
        let action = self.agent.select_action(&s);

        // 3. Apply action to consensus parameters
        self.apply_action(engine, &action);

        // 4. Let the engine run one epoch
        engine.produce_blocks();

        // 5. Observe outcome and compute reward
        let reward = self.compute_reward(engine, &s);

        // 6. Update NeuroFlux agent
        self.agent.learn(s, action, reward);

        // 7. Record metrics
        self.metrics.inc_counter("neuroflux_ticks", 1);
        self.metrics.record_observation(&s);
        self.metrics.record_reward(reward);
    }

    fn collect_state(&self, engine: &ConsensusEngine) -> QNum {
        // Pack tip_count, avg_latency, cognitive_entropy, fork_rate into a single QNum
        let vals = vec![
            engine.tip_count as u8,
            engine.avg_latency_ms as u8,
            engine.cognitive_entropy as u8,
            (engine.fork_rate * 100.0) as u8,
        ];
        QNum::from_digits(&vals)
    }

    fn apply_action(&self, engine: &mut ConsensusEngine, action: &Action) {
        let cfg = &mut engine.config;
        cfg.entropy_finality = (cfg.entropy_finality + action.delta_entropy)
            .clamp(cfg.min_entropy, cfg.max_entropy);
        cfg.max_tips = ((cfg.max_tips as isize) + action.delta_tips)
            .clamp(cfg.min_tips as isize, cfg.max_tips as isize) as usize;
    }

    fn compute_reward(&self, engine: &ConsensusEngine, prev_state: &QNum) -> f64 {
        let tps_ratio = engine.measured_tps as f64 / engine.target_tps as f64;
        let lat_penalty = engine.avg_latency_ms / engine.max_latency_ms;
        let fork_penalty = engine.fork_rate;
        engine.config.reward_weights.tps * tps_ratio
            - engine.config.reward_weights.latency * lat_penalty
            - engine.config.reward_weights.forks * fork_penalty
    }
}
```

---

## Configuration

In your `qmesh.toml`, enable and configure NeuroFlux:

```toml
# qmesh.toml
[neuroflux]
enabled             = true
learning_rate       = 0.01
discount_factor     = 0.99
exploration_rate    = 0.1

[consensus]
min_entropy         = 1.0
max_entropy         = 50.0
min_tips            = 2
max_tips            = 16

[reward_weights]
tps                 = 0.7
latency             = 0.2
forks               = 0.1
```

---

## Metrics & Monitoring

The module exposes Prometheus metrics via `ConsensusMetrics`:

* `neuroflux_initialized`
* `neuroflux_ticks`
* `consensus_observations_total`
* `consensus_rewards_sum`

Use `engine.export_metrics()` to retrieve the text format.

---

## Testing

Unit tests in `runtime/tests/consensus_neuroflux_tests.rs` verify:

* State packing and unpacking.
* Action application clamping.
* Reward calculation for synthetic scenarios.

Example:

```rust
#[test]
fn reward_calculation_matches_manual() {
    let mut engine = ConsensusEngine::mock();
    engine.measured_tps = 900;
    engine.target_tps = 1000;
    engine.avg_latency_ms = 50.0;
    engine.max_latency_ms = 100.0;
    engine.fork_rate = 0.02;
    engine.config.reward_weights = RewardWeights { tps: 1.0, latency: 1.0, forks: 1.0 };

    let mut nf = ConsensusNeuroFlux::new(&engine.config);
    let state = nf.collect_state(&engine);
    let r = nf.compute_reward(&engine, &state);

    // expected = 0.9 - 0.5 - 0.02 = 0.38
    assert!((r - 0.38).abs() < 1e-6);
}
```

---

## Usage Example

In your node startup:

```rust
let mut engine = ConsensusEngine::new(&cfg);
let mut neuroflux = ConsensusNeuroFlux::new(&cfg);

loop {
    neuroflux.tick(&mut engine);
    // other runtime tasks...
}
```

This loop continuously adapts consensus parameters to maximize network performance.

---

*End of consensus\_neuroflux.md*\`\`\`
