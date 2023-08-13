use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use tracing::error;

/// Struct representing the configurations.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Configs {
    pub name: String,
    pub port: u16,
    pub mode: Mode,
    pub db_host: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub db_namespace: String,
}

/// Enum representing the different modes.
#[derive(Debug, Deserialize)]
pub enum Mode {
    // ! find a way to make it camelcase and match the enum at the same time
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
    fn new() -> Result<Self, ConfigError> {
        // build configs
        let configs = Config::builder()
            .add_source(File::with_name("config").required(false))
            .add_source(Environment::with_prefix("APP").separator("_"))
            .add_source(Environment::default())
            .build()?;

        // return deserialized configs
        configs.try_deserialize::<Configs>()
    }

    /// Initializes the configurations.
    ///
    /// This function creates a new instance of the `Configs` struct and returns it.
    /// If an error occurs during the initialization process, an error message is logged
    /// and the program exits with a status code of 1.
    pub fn init_configs() -> Self {
        match Configs::new() {
            Ok(c) => c,
            Err(err) => {
                error!("{:#?}", &err);
                std::process::exit(1);
            }
        }
    }

    /// Returns the log level based on the current mode.
    ///
    /// # Returns
    ///
    /// - `tracing::Level::WARN` for `Mode::PROD`.
    /// - `tracing::Level::INFO` for `Mode::DEV`.
    /// - `tracing::Level::TRACE` for `Mode::DEBUG`.
    pub fn log_level(&self) -> tracing::Level {
        match self.mode {
            Mode::PROD => tracing::Level::WARN,
            Mode::DEV => tracing::Level::INFO,
            Mode::DEBUG => tracing::Level::TRACE,
        }
    }
}
