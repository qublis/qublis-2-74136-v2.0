````markdown
# NeuroFlux Reinforcement Learning (RL) in Qublis v2.0

This document describes the **NeuroFlux** Reinforcement Learning framework used in **Qublis v2.0** (2-74136) to optimize consensus, routing, validator behavior, and other network parameters in real time using quantum-enhanced, multi-dimensional decision making.

---

## Table of Contents

1. [Overview](#overview)  
2. [Core Concepts](#core-concepts)  
   - [Agent & Environment](#agent--environment)  
   - [State, Action, Reward](#state-action-reward)  
   - [Multi-Dimensional Learning](#multi-dimensional-learning)  
3. [NeuroFlux Architecture](#neuroflux-architecture)  
   - [4D: Classical RL](#4d-classical-rl)  
   - [5D: Self-Modifying Smart Contracts](#5d-self-modifying-smart-contracts)  
   - [6D: Quantum-Enhanced Decision Kernel](#6d-quantum-enhanced-decision-kernel)  
4. [Rust API & Integration](#rust-api--integration)  
   - [`NeuroFluxAgent`](#neurofluxagent)  
   - Configuration (`neuroflux.toml`)  
5. [Configuration Options](#configuration-options)  
6. [Metrics & Monitoring](#metrics--monitoring)  
7. [Example Usage](#example-usage)  
8. [Best Practices](#best-practices)  
9. [References](#references)  

---

## Overview

NeuroFlux is the adaptive, real-time optimization engine at the core of Qublis. It functions as an RL agent interacting with the blockchain network environment to:

- **Optimize consensus parameters** (e.g., tip selection weights, finality thresholds).  
- **Tune routing & relay strategies** for QNet/QNetX.  
- **Adjust validator voting weights** based on performance.  
- **Balance throughput vs. latency** under dynamic network conditions.  

Quantum enhancements allow NeuroFlux to evaluate and act on **exponentially large** state/action spaces efficiently, achieving near-optimal policies even in highly complex, multi-dimensional environments.

---

## Core Concepts

### Agent & Environment

- **Agent**: the NeuroFlux optimizer, embodied in the `NeuroFluxAgent` struct.  
- **Environment**: the running Qublis network (consensus engine, P2P layer, validators).  

The agent observes **state** from the network, selects **actions** to adjust parameters, and receives **rewards** based on performance metrics (TPS, finality time, error rates).

### State, Action, Reward

- **State** (`S`): vector of QNums encoding metrics like block rate, tip counts, average latency, validator uptimes.  
- **Action** (`A`): discrete or continuous changes to consensus and network parameters (e.g., adjust block weight by Δ, reroute threshold).  
- **Reward** (`R`): scalar combining normalized throughput, latency penalty, and stability bonus.

### Multi-Dimensional Learning

NeuroFlux spans several “dimensions” of decision making:

- **4D (Classical)**: standard deep RL (policy/value networks).  
- **5D (Meta-Logic)**: on-chain self-modifying QBLang contracts that adapt reward functions.  
- **6D (Quantum)**: non-local decision kernels leveraging entanglement to evaluate many possible actions in parallel.

---

## NeuroFlux Architecture

```text
+----------------------------+
|   NeuroFluxAgent (6D)     |
+------------+---------------+
             |
    observations: QNum state
             |
      +------+------+
      | 4D RL Agent |
      +-------------+
      |
 actions ↔ QNet/QMesh runtime
      |
 reward ← performance metrics
````

### 4D: Classical RL

* Implements a policy network `π(s) → a` and value network `V(s)`.
* Uses **Temporal Difference** (TD-learning) to update weights.
* Code lives in `ci_core/morphic_ai.rs` and in `sim/neuroflux_simulator.rs`.

### 5D: Self-Modifying Smart Contracts

* QBLang contracts dynamically adjust reward definitions based on governance signals.
* Example QBLang snippet:

  ```qblang
  contract update_reward_function(policy: QNum) {
      if measure(policy) > threshold {
          set_reward_scale(1.1);
      } else {
          set_reward_scale(0.9);
      }
  }
  ```

### 6D: Quantum-Enhanced Decision Kernel

* Core decision operator uses a non-local kernel `𝒦(s, a)`:

  $$
  U_{\mathrm{opt}} = \exp\!\Bigl(i \!\int\! \mathcal{K}(s,a)\,\hat{O}_{s,a}\,ds\,da\Bigr)
  $$

* Code uses `qublis_qnum::entangle` and `qadd` to explore many `a` concurrently.

---

## Rust API & Integration

### `NeuroFluxAgent`

```rust
use qublis_ci_core::NeuroFluxAgent;
use qublis_ci_core::config::CiCoreConfig;

let cfg = CiCoreConfig::load("neuroflux.toml")?;
let mut agent = NeuroFluxAgent::new(&cfg);

// In each network tick:
let state: QNum = agent.observe_network(&network_metrics);
let action: Action = agent.select_action(&state)?;
network.apply_action(&action);
let reward = agent.compute_reward(&network_metrics);
agent.learn(state, action, reward);
```

Key methods:

* `observe_network(&self, metrics: &NetworkMetrics) -> QNum`
* `select_action(&mut self, state: &QNum) -> Result<Action, NeuroFluxError>`
* `compute_reward(&self, metrics: &NetworkMetrics) -> f64`
* `learn(&mut self, s: QNum, a: Action, r: f64)`

### Configuration (`neuroflux.toml`)

```toml
# neuroflux.toml
learning_rate       = 0.01
discount_factor     = 0.99
exploration_rate    = 0.1
quantum_kernel_scale = 100.0
reward_throughput_weight = 0.7
reward_latency_penalty   = 0.3
```

---

## Configuration Options

| Option                     | Type  | Default | Description                                   |
| -------------------------- | ----- | ------- | --------------------------------------------- |
| `learning_rate`            | float | 0.01    | Base RL learning rate                         |
| `discount_factor`          | float | 0.99    | Future reward discount γ                      |
| `exploration_rate`         | float | 0.1     | ε-greedy exploration probability              |
| `quantum_kernel_scale`     | float | 100.0   | Scaling factor for quantum decision kernel    |
| `reward_throughput_weight` | float | 0.7     | Weight of TPS in composite reward             |
| `reward_latency_penalty`   | float | 0.3     | Weight of latency penalty in composite reward |

---

## Metrics & Monitoring

NeuroFlux exposes Prometheus metrics on port **9500**:

* `neuroflux_iterations_total`
* `neuroflux_rewards_sum`
* `neuroflux_actions_selected`
* `neuroflux_kernel_evaluations`

Use `qublis-ci_core`’s metrics exporter:

```rust
let metrics_txt = agent.export_metrics();
println!("{}", metrics_txt);
```

---

## Example Usage

1. **Enable NeuroFlux** in consensus config (`ci_core.toml`):

   ```toml
   enable_neuroflux = true
   ```

2. **Run the node**:

   ```bash
   qublis-node --config qmesh.toml --ci-config ci_core.toml
   ```

3. **Observe metrics**:

   ```bash
   curl localhost:9500/metrics | grep neuroflux
   ```

4. **Adjust parameters** on the fly by updating `neuroflux.toml` and sending a QBLang update:

   ```bash
   qublis-qlink-cli contract execute update_reward_function --params new_policy.qblang
   ```

---

## Best Practices

* **Start** with conservative learning rates and exploration for network stability.
* **Monitor** composite reward to detect oscillations or runaway behaviors.
* **Use** quantum kernel scale to trade off depth of lookahead vs. overhead.
* **Version** QBLang reward functions to enable rollback.

---

## References

* NeuroFlux Simulator (Rust): `sim/neuroflux_simulator.rs`
* CI-Core Integration: `ci_core/morphic_ai.rs`
* QBLang Reward Contracts: `contracts/neuroflux_reward.qblang`
* QMesh NeuroFlux Hooks: `qmesh/consensus_neuroflux.rs`

*End of neuroflux\_rl.md*\`\`\`
