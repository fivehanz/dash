use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use tracing::{debug, error};

/// Struct representing the configurations.
#[derive(Debug, Deserialize, Clone, Default)]
pub struct Configs {
    pub app_name: String,
    pub app_mode: Mode,
    pub app_port: u16,
    pub grpc_port: u16,
    pub db_host: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub db_namespace: String,
}

/// Enum representing the different modes.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Deserialize, Clone, Default)]
pub enum Mode {
    // ! find a way to make it camelcase and match the enum at the same time
    PROD,
    #[default]
    DEV,
    DEBUG,
}

impl Configs {
    /// Builds a new instance of Config struct.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the newly created instance or a `ConfigError` if there was an error.
    fn new() -> Result<Configs, ConfigError> {
        // build configs
        let read = Config::builder()
            .add_source(File::with_name("app").required(false))
            .add_source(Environment::default())
            .build();

        match read {
            Ok(config) => {
                // log configs
                debug!("read env & configs");
                // return deserialized configs
                config.try_deserialize::<Configs>()
            }
            Err(err) => Err(err),
        }
    }

    /// Initializes the configurations.
    ///
    /// This function creates a new instance of the `Configs` struct and returns it.
    /// If an error occurs during the initialization process, an error message is logged
    /// and a new default Config is returned.
    pub fn init() -> Self {
        match Self::new() {
            Ok(configs) => configs,
            Err(err) => {
                error!("config & env error: {:?}", &err);
                Self::default_with_port(4000, 4001)
            }
        }
    }

    fn default_with_port(app_port: u16, grpc_port: u16) -> Self {
        Self {
            app_port,
            grpc_port,
            ..Self::default()
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
        match self.app_mode {
            Mode::PROD => tracing::Level::INFO,
            Mode::DEV => tracing::Level::DEBUG,
            Mode::DEBUG => tracing::Level::TRACE,
        }
    }
}
