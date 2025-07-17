//! WASM smart‐contract executor for QBLang modules in Qublis v2.0 (2-74136).
//!
//! Provides `WasmExecutor` to compile, instantiate, and invoke WebAssembly
//! modules emitted by the QBLang compiler, with configurable memory and gas limits.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use crate::config::WasmConfig;
use thiserror::Error;
use wasmtime::{Engine, Module, Store, Instance, Func, Linker, Caller, Val};

/// Errors that can occur during WASM compilation or execution.
#[derive(Debug, Error)]
pub enum WasmError {
    /// Failed to compile the WASM module.
    #[error("WASM compile error: {0}")]
    Compile(String),
    /// Failed to instantiate the WASM module.
    #[error("WASM instantiation error: {0}")]
    Instantiate(String),
    /// The requested function was not found.
    #[error("Function `{0}` not found in module")]
    FunctionNotFound(String),
    /// Failed while calling the exported function.
    #[error("WASM execution error: {0}")]
    Execution(String),
}

/// Executor for QBLang‐compiled WASM contracts.
#[derive(Clone)]
pub struct WasmExecutor {
    engine: Engine,
    cfg: WasmConfig,
}

impl WasmExecutor {
    /// Create a new executor with the given WASM configuration.
    ///
    /// # Arguments
    ///
    /// * `cfg` – contains `memory_limit` and `gas_limit` for instances.
    pub fn new(cfg: &WasmConfig) -> Self {
        // In a real implementation we might configure fuel & memory via StoreLimits
        let engine = Engine::default();
        WasmExecutor {
            engine,
            cfg: cfg.clone(),
        }
    }

    /// Execute an exported function in the given WASM module.
    ///
    /// Loads, instantiates, and invokes `func_name` with the provided parameters.
    /// Returns the optional single `Val` result (or `None` for void).
    ///
    /// # Arguments
    ///
    /// * `wasm_bytes` – raw `.wasm` module bytes.  
    /// * `func_name` – the name of the exported function to call.  
    /// * `params` – a slice of WASM `Val` parameters.
    pub fn execute(
        &self,
        wasm_bytes: &[u8],
        func_name: &str,
        params: &[Val],
    ) -> Result<Option<Val>, WasmError> {
        // 1. Compile
        let module = Module::from_binary(&self.engine, wasm_bytes)
            .map_err(|e| WasmError::Compile(e.to_string()))?;

        // 2. Create store (with no initial host state)
        let mut store = Store::new(&self.engine, ());

        // 3. Instantiate
        //    In real code, we'd configure memory & gas via resource limits here
        let mut linker = Linker::new(&self.engine);
        let instance = linker
            .instantiate(&mut store, &module)
            .map_err(|e| WasmError::Instantiate(e.to_string()))?;

        // 4. Lookup function
        let func = instance
            .get_func(&mut store, func_name)
            .ok_or_else(|| WasmError::FunctionNotFound(func_name.to_string()))?;

        // 5. Call
        //    Support up to one return value for simplicity
        let results = func
            .call(&mut store, params)
            .map_err(|e| WasmError::Execution(e.to_string()))?;

        // 6. Return first result or None
        Ok(results.into_iter().next())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasmtime::Val;

    /// A minimal Wasm module (compiled from WAT) that exports `add_one`.
    const ADD_ONE_WASM: &[u8] = &wat::parse_str(r#"
        (module
          (func $add_one (export "add_one") (param i32) (result i32)
            local.get 0
            i32.const 1
            i32.add)
        )
    "#).unwrap();

    #[test]
    fn execute_add_one_success() {
        let cfg = WasmConfig {
            memory_limit: 64 * 1024 * 1024,
            gas_limit: 1_000_000,
        };
        let exec = WasmExecutor::new(&cfg);
        let param = Val::I32(41);
        let result = exec.execute(ADD_ONE_WASM, "add_one", &[param]).unwrap();
        assert_eq!(result, Some(Val::I32(42)));
    }

    #[test]
    fn missing_function_returns_error() {
        let cfg = WasmConfig::default();
        let exec = WasmExecutor::new(&cfg);
        let err = exec.execute(ADD_ONE_WASM, "nonexistent", &[]).unwrap_err();
        match err {
            WasmError::FunctionNotFound(name) => assert_eq!(name, "nonexistent"),
            _ => panic!("expected FunctionNotFound"),
        }
    }

    #[test]
    fn invalid_module_compile_error() {
        let cfg = WasmConfig::default();
        let exec = WasmExecutor::new(&cfg);
        let bad = b"\0\x61\x73\x6d"; // invalid WASM header
        let err = exec.execute(bad, "foo", &[]).unwrap_err();
        match err {
            WasmError::Compile(_) => {}
            _ => panic!("expected Compile error"),
        }
    }
}
