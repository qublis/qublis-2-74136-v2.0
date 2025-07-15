//! Deployment Config Loader for Qublis v2.0
//!
//! Provides generic helpers to load TOML configuration files for deployment tools.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use serde::de::DeserializeOwned;
use std::{fs, path::Path};
use thiserror::Error;

/// Errors that can occur while loading a configuration file.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// I/O error reading the file.
    #[error("I/O error reading config file: {0}")]
    Io(#[from] std::io::Error),

    /// TOML deserialization error.
    #[error("TOML parse error: {0}")]
    Parse(#[from] toml::de::Error),
}

/// Load a TOML configuration file from `path` into the provided struct `T`.
///
/// # Examples
///
/// ```no_run
/// use deploy::config::{load_toml_config, ConfigError};
///
/// #[derive(serde::Deserialize)]
/// struct MyConfig { field: String }
///
/// fn main() -> Result<(), ConfigError> {
///     let cfg: MyConfig = load_toml_config("my_config.toml")?;
///     println!("Loaded field: {}", cfg.field);
///     Ok(())
/// }
/// ```
pub fn load_toml_config<T, P>(path: P) -> Result<T, ConfigError>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let s = fs::read_to_string(&path)?;
    let cfg = toml::from_str(&s)?;
    Ok(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestConfig {
        foo: String,
        bar: i32,
    }

    #[test]
    fn load_valid_config() {
        let toml = r#"
            foo = "hello"
            bar = 42
        "#;
        let mut file = NamedTempFile::new().expect("create temp file");
        write!(file, "{}", toml).expect("write toml");
        let cfg: TestConfig = load_toml_config(file.path()).expect("load config");
        assert_eq!(cfg, TestConfig { foo: "hello".into(), bar: 42 });
    }

    #[test]
    fn missing_file_errors_io() {
        let err = load_toml_config::<TestConfig, _>("nonexistent.toml").unwrap_err();
        matches!(err, ConfigError::Io(_));
    }

    #[test]
    fn invalid_toml_errors_parse() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "not = valid = toml").unwrap();
        let err = load_toml_config::<TestConfig, _>(file.path()).unwrap_err();
        matches!(err, ConfigError::Parse(_));
    }
}
