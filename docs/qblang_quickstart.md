````markdown
# QBLang Quickstart

Welcome to **QBLang**, the domain-specific smart-contract language for Qublis v2.0. In just a few steps you'll learn how to write, compile, test, and deploy QBLang contracts that integrate seamlessly with the Qublis blockchain and runtime.

---

## Prerequisites

- **Rust 1.65+** and Cargo (for QBLang CLI tool)
- **Qublis v2.0** workspace checked out locally
- `qublis-qblang-cli` installed (provided in the workspace)

```bash
# From the workspace root
cd qublis-2-74136-v2.0
cargo install --path qblang/cli
````

This installs the `qublis-qblang` CLI executable.

---

## 1. Hello, QBLang!

Create a new file `contracts/hello.qblang`:

```qblang
// hello.qblang
contract hello_world() -> String {
    // Return a greeting
    return "Hello, QBLang v2.0!";
}
```

---

## 2. Compile Your Contract

Use the CLI to compile to WASM:

```bash
qublis-qblang compile \
  --input contracts/hello.qblang \
  --output artifacts/hello_world.wasm
```

* `--input`: path to your `.qblang` file
* `--output`: path to the generated WebAssembly module

On success you’ll see:

```
Compiled hello.qblang → artifacts/hello_world.wasm
```

---

## 3. Test Locally

Write a simple JSON test vector in `tests/hello_test.json`:

```json
{
  "contract": "hello_world",
  "method": "hello_world",
  "params": []
}
```

Run the QBLang VM in test mode:

```bash
qublis-qblang test \
  --wasm artifacts/hello_world.wasm \
  --input tests/hello_test.json
```

Expected output:

```
TEST PASS: hello_world() -> "Hello, QBLang v2.0!"
```

---

## 4. Deploy On-Chain

Assuming you have a running Qublis node with the `qublis-qlink-cli` installed:

1. **Upload** the WASM to storage:

   ```bash
   qublis-qlink-cli upload-wasm \
     --file artifacts/hello_world.wasm \
     --name hello_world \
     --version 1
   ```

2. **Instantiate** the contract:

   ```bash
   qublis-qlink-cli instantiate-contract \
     --wasm-name hello_world \
     --wasm-version 1 \
     --contract-name hello_world \
     --constructor hello_world \
     --args '[]'
   ```

3. **Call** your contract:

   ```bash
   qublis-qlink-cli call \
     --contract hello_world \
     --method hello_world \
     --args '[]'
   ```

You should see the on-chain response:

```
CALL RESULT: "Hello, QBLang v2.0!"
```

---

## 5. Working with Parameters

Define a contract with parameters in `contracts/add.qblang`:

```qblang
// add.qblang
contract add(a: u64, b: u64) -> u64 {
    // Return the sum
    return a + b;
}
```

Compile, test, and deploy:

```bash
qublis-qblang compile --input contracts/add.qblang --output artifacts/add.wasm

echo '{
  "contract": "add",
  "method": "add",
  "params": [5, 7]
}' > tests/add_test.json

qublis-qblang test --wasm artifacts/add.wasm --input tests/add_test.json
# → TEST PASS: add(5,7) -> 12

# On-chain call:
qublis-qlink-cli call \
  --contract add \
  --method add \
  --args '[5,7]'
# → CALL RESULT: 12
```

---

## 6. Events & Logging

Emit events with the `emit` keyword:

```qblang
// events.qblang
contract counter() -> () {
    let count = 0u64;
    for i in 1..=3 {
        count = count + i;
        emit("count_updated", count);
    }
    return;
}
```

* `emit(name: string, data: QNum|primitive)`: logs an event.

Test locally:

```bash
qublis-qblang test \
  --wasm artifacts/events.wasm \
  --input '{
    "contract": "counter",
    "method": "counter",
    "params": []
  }'
```

You will see:

```
EVENT count_updated: 1
EVENT count_updated: 3
EVENT count_updated: 6
TEST PASS: counter() -> ()
```

---

## 7. Using QNums

QBLang supports `QNum` as first-class type:

```qblang
// qnum_example.qblang
import qnum;

contract fib(n: u32) -> QNum {
    // Compute Fibonacci using QNums
    if n <= 1 {
        return QNum::from(n);
    }
    let a = QNum::from(0u32);
    let b = QNum::from(1u32);
    let i = 2u32;
    let mut x = a;
    let mut y = b;
    while i <= n {
        let tmp = x + y;
        x = y;
        y = tmp;
        i = i + 1;
    }
    return y;
}
```

Compile & test:

```bash
qublis-qblang compile --input contracts/qnum_example.qblang \
                     --output artifacts/fib.wasm

echo '{
  "contract":"fib",
  "method":"fib",
  "params":[10]
}' > tests/fib_test.json

qublis-qblang test --wasm artifacts/fib.wasm --input tests/fib_test.json
# → TEST PASS: fib(10) -> 55
```

---

## 8. Advanced: Self-Modifying Contracts (5D)

QBLang can modify its own code using on-chain state and `eval`:

```qblang
// self_mod.qblang
contract upgrader(flag: bool) -> () {
    if flag {
        eval(r#"
        contract hello() -> String {
            return "Upgraded!";
        }"#);
    }
    return;
}
```

1. Deploy `self_mod.wasm` and call `upgrader(true)`.
2. A new `hello` contract will be registered on-chain.

---

## 9. Resources & Next Steps

* **Full QBLang Spec**: `docs/qblang_spec.md`
* **Smart-Contract Patterns**: `docs/qblang_patterns.md`
* **Security Guidelines**: `docs/qblang_security.md`

---

With this quickstart you’re ready to start building powerful, quantum- and AI-integrated smart contracts on Qublis v2.0. Happy coding!
