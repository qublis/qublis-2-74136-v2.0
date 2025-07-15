```markdown
# Qublis v2.0 Deployment Guide

This guide walks you through deploying **Qublis v2.0** (2-74136) in production, covering prerequisites, topology design, node bootstrapping, CI/CD integration, monitoring, upgrades, and security best practices.

---

## Table of Contents

1. [Prerequisites](#prerequisites)  
2. [Hardware & OS Recommendations](#hardware--os-recommendations)  
3. [Network Topology](#network-topology)  
4. [Configuration Files](#configuration-files)  
5. [Bootstrapping Nodes](#bootstrapping-nodes)  
   - [CI Fork Launcher](#ci-fork-launcher)  
   - [QNetX Node Bootstrap](#qnetx-node-bootstrap)  
6. [Continuous Deployment](#continuous-deployment)  
7. [Monitoring & Logging](#monitoring--logging)  
8. [Upgrades & Migration](#upgrades--migration)  
9. [Security Hardening](#security-hardening)  
10. [Troubleshooting](#troubleshooting)  

---

## Prerequisites

- **Git** (≥2.35)  
- **Rust** toolchain (via `rustup`, nightly optional)  
- **Docker** & **Docker Compose** (for local testing)  
- **Tokio** runtime for async tools  
- **Prometheus**, **Grafana**, **Alertmanager** (optional but recommended)  
- **Access** to machines/VMs (bare-metal or cloud) with:
  - Public IPv4/IPv6 addresses  
  - Low-latency network connectivity  

---

## Hardware & OS Recommendations

| Component          | Minimum          | Recommended                   |
|--------------------|------------------|-------------------------------|
| CPU                | 8 cores          | 32+ cores (for QNetX heavy)   |
| RAM                | 16 GB            | 64 GB                         |
| Storage            | 500 GB SSD       | 2 TB NVMe                     |
| Network            | 1 Gbps           | 10 Gbps (for >10M TPS)        |
| OS                 | Ubuntu 20.04 LTS | Ubuntu 22.04 LTS / CentOS 9   |
| Kernel tuning      | default          | tuned for high concurrency    |

---

## Network Topology

```

```
       ┌────────────┐
       │  Load      │
       │ Balancer   │
       └────┬───────┘
            │
  ┌─────────┴─────────┐
  │                   │
```

┌─────▼─────┐       ┌─────▼─────┐
│ Validator │  ...  │ Validator │
│  Node A   │       │  Node B   │
└───────────┘       └───────────┘
│                   │
└─────┬─────┬───────┘
│     │
┌─────▼─────▼─────┐
│ QNetX Overlay   │
│ (Meshing & DAG) │
└─────────────────┘

```

- **Validators** run `qublis-qnetx-node`.  
- **Subnets** can be used for geographic sharding.  
- **Entrypoints**: expose gRPC/JSON-RPC on secured ports.

---

## Configuration Files

Store your deployment configs under a versioned directory:

```

deploy/
├── ci\_forks.toml
├── bootstrap.toml
└── qnetx\_nodes/
├── nodeA.toml
└── nodeB.toml

````

### Example: `ci_forks.toml`

```toml
upstream_repo = "https://github.com/qublis/qublis-2-74136-v2.0.git"
branch        = "main"
forks         = 3
output_dir    = "./ci_forks"
run_tests     = true
````

### Example: `bootstrap/nodeA.toml`

```toml
node_name       = "validator-A"
listen_addr     = "/ip4/0.0.0.0/tcp/30333"
bootstrap_peers = [ "/ip4/10.0.0.2/tcp/30333" ]
chain_spec      = "../specs/chainSpec.json"
base_path       = "/var/lib/qublis/nodeA"
dev_mode        = false
```

---

## Bootstrapping Nodes

### CI Fork Launcher

Run parallel forks for CI validation:

```bash
qublis-deploy ci-fork-launcher \
  --config deploy/ci_forks.toml
```

This clones forks into `./ci_forks/fork_{1..3}`, checks out `main`, and runs `cargo test`.

### QNetX Node Bootstrap

Prepare and launch a QNetX node:

```bash
qublis-deploy qnetx-node-bootstrap \
  --config deploy/bootstrap/nodeA.toml
```

This will:

1. Create `base_path` directory.
2. Copy the chain spec.
3. Invoke `qublis-qnetx-node` with appropriate flags.

---

## Continuous Deployment

Integrate with GitHub Actions (`.github/workflows/ci.yml` / `release.yml`):

* **CI**:

  * Build & test all crates (`cargo test --workspace`).
  * Lint & format checks (`cargo fmt -- --check`).
  * QBLang compile & test flow.

* **Release**:

  * Tag & publish docs.
  * Upload artifacts (WASM, binaries) to GitHub Releases.
  * Trigger on-chain upgrade via `qublis-qlink-cli`.

---

## Monitoring & Logging

1. **Prometheus Exporters**: each crate exposes metrics on its port (`9100`–`9500`).
2. **Scrape Targets**:

   ```yaml
   scrape_configs:
     - job_name: 'qublis'
       static_configs:
         - targets:
           - 'validator-A:9300'
           - 'validator-B:9310'
   ```
3. **Dashboards**: import provided Grafana dashboards in `deploy/grafana/`.
4. **Logs**: run nodes under `systemd`:

   ```ini
   [Unit]
   Description=Qublis QNetX Node
   After=network.target

   [Service]
   User=qublis
   ExecStart=/usr/local/bin/qublis-qnetx-node \
     --config /etc/qublis/nodeA.toml
   Restart=on-failure
   StandardOutput=journal
   StandardError=journal

   [Install]
   WantedBy=multi-user.target
   ```

---

## Upgrades & Migration

1. **Prepare** new binaries (`v2.1.0`), update `VERSION`.
2. **CI**: test all forked repos with new code.
3. **Drain** traffic from old validators via load-balancer.
4. **Stop** old service, replace binary, restart.
5. **ChainSpec Upgrade** (if needed):

   ```bash
   qublis-qlink-cli contract execute upgrade_chainSpec \
     --args '["newSpecHash"]'
   ```
6. **Validate** health & metrics post-upgrade.

---

## Security Hardening

* **Firewall**: restrict ports to known peers.
* **TLS**: terminate at load balancer or use `--tls` flags.
* **Key Management**: store node keys in `~/.local/share/qublis/keystore`; use HSM if available.
* **Audit**: enable audit logs on `qlink` contract execution.

---

## Troubleshooting

| Issue                          | Resolution                                                  |
| ------------------------------ | ----------------------------------------------------------- |
| Node fails to start            | Check config file path, permissions, and `chain_spec` path. |
| High fork rate                 | Tune `max_tips` / `entropy_finality` via `ci_core` config.  |
| Low TPS reported               | Monitor `sim` suite offline; adjust network MTU or ulimit.  |
| QBLang contract compile errors | Ensure CLI version matches contract spec version.           |
| Metrics missing                | Verify ports and Prometheus scrape config.                  |

---

Congratulations! You have now deployed and configured Qublis v2.0 for production. For further details, refer to the individual crate docs under `docs/`.
