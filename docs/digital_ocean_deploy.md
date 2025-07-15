````markdown
# Deploying a Qublis v2.0 Validator Node on DigitalOcean

This guide walks you through creating and configuring an Ubuntu droplet on DigitalOcean to run a **Qublis v2.0** validator (QNetX) node in 2-74136.

---

## Prerequisites

- A DigitalOcean account with API access (or use the Control Panel).  
- SSH key pair added to your DigitalOcean account.  
- Basic familiarity with Linux command line and `rustup`.  

---

## 1. Create the Droplet

1. **Log in** to DigitalOcean → **Create → Droplets**.  
2. Choose **Ubuntu 22.04 LTS**.  
3. Select a plan:
   - **Standard**: 8 CPU, 32 GB RAM, 160 GB SSD (minimum for >10M TPS).  
4. Add your SSH key.  
5. Choose a hostname, e.g. `qublis-valid-1`.  
6. Enable **Monitoring** (optional).  
7. Click **Create Droplet**.

---

## 2. Firewall & Networking

1. In the Control Panel, go to **Networking → Firewalls**.  
2. Create a firewall allowing:
   - **TCP** 30333 (QNetX P2P)  
   - **TCP** 9944 (JSON-RPC)  
   - **TCP** 9100–9500 (Prometheus ports)  
   - **SSH** (port 22) from your IP.  
3. Apply the firewall to your droplet.

---

## 3. Initial Server Setup

SSH into your droplet:

```bash
ssh root@<DROPLET_IP>
````

Update & install essentials:

```bash
apt update && apt upgrade -y
apt install -y build-essential curl git pkg-config libssl-dev
```

---

## 4. Install Rust & Qublis Dependencies

Install Rust (stable toolchain):

```bash
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
rustup default stable
rustup target add wasm32-unknown-unknown
```

Clone the Qublis v2.0 workspace:

```bash
cd /opt
git clone https://github.com/qublis/qublis-2-74136-v2.0.git
chown -R root:root qublis-2-74136-v2.0
```

---

## 5. Build the QNetX Node Binary

```bash
cd /opt/qublis-2-74136-v2.0
cargo build --release -p qublis-qnetx-node
# Binary at: target/release/qublis-qnetx-node
```

(Optional) Repeat for any other binaries you need, e.g. `qublis-node`, `qublis-sim`.

---

## 6. Prepare Configuration

Create `~/qnetx_node.toml` (or `/etc/qublis/node.toml`):

```toml
node_name       = "validator-1"
listen_addr     = "/ip4/0.0.0.0/tcp/30333"
bootstrap_peers = [
  "/ip4/203.0.113.5/tcp/30333",
  "/ip4/203.0.113.6/tcp/30333"
]
chain_spec      = "/opt/qublis-2-74136-v2.0/specs/chainSpec.json"
base_path       = "/var/lib/qublis/node1"
dev_mode        = false
```

Fetch or place your `chainSpec.json` at the path above.

---

## 7. Create Systemd Service

Create `/etc/systemd/system/qublis-qnetx-node.service`:

```ini
[Unit]
Description=Qublis v2.0 QNetX Validator Node
After=network.target

[Service]
User=root
ExecStart=/opt/qublis-2-74136-v2.0/target/release/qublis-qnetx-node \
  --config /root/qnetx_node.toml
Restart=on-failure
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
```

Reload and enable:

```bash
systemctl daemon-reload
systemctl enable qublis-qnetx-node
systemctl start qublis-qnetx-node
```

---

## 8. Monitoring & Logs

* **Logs**:

  ```bash
  journalctl -u qublis-qnetx-node -f
  ```
* **Prometheus**: scrape metrics on port configured in your node (default 9200).
* **Grafana**: import dashboards from `deploy/grafana/`.

---

## 9. Optional: Automate with `qublis-deploy`

You can use the built‐in deploy tooling:

```bash
# Ensure you have a bootstrap TOML at ~/bootstrap_node1.toml
qublis-deploy qnetx-node-bootstrap --config ~/bootstrap_node1.toml
```

This will create the data directory, copy the spec, and launch the node.

---

## 10. Upgrades & Maintenance

To upgrade:

1. **Stop** the service:

   ```bash
   systemctl stop qublis-qnetx-node
   ```
2. **Pull** latest code & rebuild:

   ```bash
   cd /opt/qublis-2-74136-v2.0
   git pull origin main
   cargo build --release -p qublis-qnetx-node
   ```
3. **Restart**:

   ```bash
   systemctl start qublis-qnetx-node
   ```

---

Congratulations! Your Qublis v2.0 validator is now running on DigitalOcean. For further tuning (NeuroFlux, QMesh parameters, etc.), consult the `docs/` in the repository.
