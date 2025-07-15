```markdown
# QBLang v2.0 Language Specification

This document defines the syntax, semantics, and high-level architecture of **QBLang v2.0**, the domain-specific smart-contract language for Qublis v2.0 (2-74136). QBLang is designed to express on-chain logic, quantum-native data types, entangled computations, and AI-driven self-modifying contracts.

---

## Table of Contents

1. [Design Goals](#design-goals)  
2. [Source File Structure](#source-file-structure)  
3. [Lexical Structure](#lexical-structure)  
   - [Comments](#comments)  
   - [Identifiers](#identifiers)  
   - [Literals](#literals)  
   - [Operators & Punctuation](#operators--punctuation)  
4. [Types](#types)  
   - [Primitive Types](#primitive-types)  
   - [Quantum Types](#quantum-types)  
   - [Composite Types](#composite-types)  
5. [Declarations](#declarations)  
   - [Imports](#imports)  
   - [Contracts](#contracts)  
   - [Functions](#functions)  
   - [Structs & Enums](#structs--enums)  
6. [Control Flow](#control-flow)  
   - [If / Else](#if--else)  
   - [Loops](#loops)  
   - [Match Expressions](#match-expressions)  
7. [Expressions & Operators](#expressions--operators)  
8. [Events & Logging](#events--logging)  
9. [Error Handling](#error-handling)  
10. [Special Features](#special-features)  
    - [Quantum Operations](#quantum-operations)  
    - [Self-Modifying Code (5D)](#self-modifying-code-5d)  
    - [Contract Upgrades](#contract-upgrades)  
11. [Module & Namespace Rules](#module--namespace-rules)  
12. [Compilation & Deployment](#compilation--deployment)  
13. [Examples](#examples)  

---

## Design Goals

- **Quantum-Native**: First-class support for **QNum**, **Qid**, entanglement, and measurement.  
- **Deterministic**: All contract executions produce deterministic results given same inputs.  
- **Self-Modifying**: Contracts may deploy or modify other contracts at runtime.  
- **Type-Safe**: Static type checking with minimal overhead in WASM.  
- **Familiar Syntax**: Inspired by Rust and Solidity for readability.  
- **AI-Integrated**: Hooks for NeuroFlux reward definitions and logical evolution.

---

## Source File Structure

```

contracts/
├── my\_contract.qblang
├── utils.qblang
└── modules/
└── math.qblang

````

- **`.qblang`** files contain one or more **contract** or **module** definitions.
- Files are compiled to WebAssembly (`.wasm`) and deployed via the Qublis toolchain.

---

## Lexical Structure

### Comments

- Single-line: `// this is a comment`  
- Multi-line: `/* this spans \n multiple lines */`

### Identifiers

- Begin with letter or `_`, followed by letters, digits, or `_`.  
- Case-sensitive.

### Literals

- **Integers**: `42`, `0x2A`, `0o52`, `0b101010`  
- **Floats**: `3.14`, `1e-3`  
- **Strings**: `"hello"`, `'world'`  
- **Booleans**: `true`, `false`  
- **QNum Literals**: `QNum::from_digits([1,2,3])` or shorthand `#123`.  
- **Byte Arrays**: `b"0xFFEE"` or `[0xFF, 0xEE]`.

### Operators & Punctuation

| Category         | Symbols                         |
|------------------|---------------------------------|
| Arithmetic       | `+`, `-`, `*`, `/`, `%`, `**`   |
| Comparison       | `==`, `!=`, `<`, `<=`, `>`, `>=`|
| Logical          | `&&`, `||`, `!`                 |
| Bitwise          | `&`, `|`, `^`, `<<`, `>>`       |
| Quantum          | `⊕` (qadd), `⊗` (qmul)           |
| Assignment       | `=`, `+=`, `-=`, etc.           |
| Separator        | `;`, `,`, `:`                   |
| Delimiters       | `(`, `)`, `{`, `}`, `[`, `]`    |
| Others           | `->`, `=>`, `.`                 |

---

## Types

### Primitive Types

- `u8, u16, u32, u64, u128`  
- `i8, i16, i32, i64, i128`  
- `bool`  
- `String`  
- `Bytes`  

### Quantum Types

- `Qid` — quantum identity, alias for fixed-length `QNum`.  
- `QNum` — superposed numeric type with entanglement support.  

### Composite Types

- **Arrays**: `[T; N]`  
- **Vectors**: `Vec<T>`  
- **Option**: `Option<T>`  
- **Result**: `Result<T, E>`  
- **Structs** & **Enums**

---

## Declarations

### Imports

```qblang
import math;
import utils::{foo, bar};
import qnum::{QNum, measure};
````

### Contracts

```qblang
contract MyContract(param1: u64, param2: QNum) -> QNum {
    // stateful or stateless logic
    return param2 + QNum::from(param1);
}
```

* **Entry point** is the contract name.
* **Parameters** declared with names and types.
* **Return type** after `->`.

### Functions

```qblang
fn helper(x: u32) -> u32 {
    x * x
}
```

* Private, reusable within file or module.

### Structs & Enums

```qblang
struct Point { x: i32, y: i32 }
enum Status { Pending, Approved, Rejected(String) }
```

---

## Control Flow

### If / Else

```qblang
if cond {
    // ...
} else if other {
    // ...
} else {
    // ...
}
```

### Loops

* **While**: `while cond { ... }`
* **For**: `for i in 0..10 { ... }`
* **Loop**: `loop { ... break; }`

### Match Expressions

```qblang
match status {
  Status::Pending => ...,
  Status::Approved => ...,
  Status::Rejected(msg) => ...,
}
```

---

## Expressions & Operators

* Standard expression precedence.
* Quantum operators:

  * `a ⊕ b` — quantum addition (unitary).
  * `a ⊗ b` — quantum multiplication (entanglement).
* Measurement: `let c: u64 = measure(qnum);`

---

## Events & Logging

```qblang
emit("Transfer", from, to, amount);
```

* `emit(name: string, ...args)` publishes on-chain event.

---

## Error Handling

* **Revert**: stop execution and rollback:

  ```qblang
  require(balance >= amount, "Insufficient funds");
  ```

* `panic!("message")` — abort with message.

---

## Special Features

### Quantum Operations

```qblang
let ψ: QNum = entangle(q1, q2);
let measured: u64 = measure(ψ);
```

* `entangle(a, b)` — entangles two `QNum`s.
* `collapse(q)` — alias for `measure(q)`.

### Self-Modifying Code (5D)

```qblang
eval(r#"
contract upgraded() -> String {
  return "New logic!";
}
"#);
```

* `eval(source: string)` compiles & registers new contract at runtime.

### Contract Upgrades

* Upgradable contracts maintain a **proxy** pattern:

  * `delegate_call(new_impl)` to switch logic.

---

## Module & Namespace Rules

* **One namespace** per file.
* Contracts and functions share same namespace.
* Import collisions must be qualified.

---

## Compilation & Deployment

1. **Compile**:

   ```bash
   qublis-qblang compile --input file.qblang --output file.wasm
   ```
2. **Test**:

   ```bash
   qublis-qblang test --wasm file.wasm --input test.json
   ```
3. **Deploy**: via `qublis-qlink-cli` or `qublis-deploy`.

---

## Examples

### Simple Token

```qblang
contract Token(initial_supply: u64) -> () {
    let total = QNum::from(initial_supply);
    emit("Minted", total);
}

fn transfer(from: Qid, to: Qid, amt: u64) -> bool {
    // ...
    emit("Transfer", from, to, QNum::from(amt));
    return true;
}
```

---

*End of QBLang v2.0 Specification*

```
::contentReference[oaicite:0]{index=0}
```
