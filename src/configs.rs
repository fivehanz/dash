use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;

/// Struct representing the configurations.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Configs {
    pub name: String,
    pub port: u16,
    database_url: String,
    pub mode: Mode,
}

/// Enum representing the different modes.
#[derive(Debug, Deserialize)]
pub enum Mode {
    PROD,
    DEV,
    DEBUG,
}

impl Configs {
    /// Creates a new instance of the struct.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the newly created instance or a `ConfigError` if there was an error.
    pub fn new() -> Result<Self, ConfigError> {
        // build configs
        let configs = Config::builder()
            .add_source(Environment::with_prefix("APP"))
            .add_source(File::with_name("config"))
            .build()?;

        // return deserialized configs
        configs.try_deserialize::<Configs>()
    }

    /// Returns the log level based on the current mode.
    ///
    /// # Returns
    ///
    /// - `tracing::Level::WARN` for `Mode::PROD`.
    /// - `tracing::Level::INFO` for `Mode::DEV`.
    /// - `tracing::Level::TRACE` for `Mode::DEBUG`.
    pub fn log_level(&self) -> tracing::Level {
        return match self.mode {
            Mode::PROD => tracing::Level::WARN,
            Mode::DEV => tracing::Level::INFO,
            Mode::DEBUG => tracing::Level::TRACE,
        };
    }
}
