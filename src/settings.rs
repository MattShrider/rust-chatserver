use figment::{
    providers::{Env, Format, Serialized, Toml},
    Error, Figment,
};
use serde::{Deserialize, Serialize};
use tracing::Level;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum LogLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

impl From<&LogLevel> for Level {
    fn from(value: &LogLevel) -> Self {
        match value {
            LogLevel::TRACE => Level::TRACE,
            LogLevel::DEBUG => Level::DEBUG,
            LogLevel::INFO => Level::INFO,
            LogLevel::WARN => Level::WARN,
            LogLevel::ERROR => Level::ERROR,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Global settings loaded in from Config.toml
pub struct Settings {
    /// The web server is hosted at this name
    pub host: String,
    /// The web server is hosted at this port
    pub port: String,
    /// Controls the log level used by the tracing crate
    pub log_level: LogLevel,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: "3000".into(),
            log_level: LogLevel::WARN,
        }
    }
}

/// Creates application settings read in from configuration files.
/// Config can be overridden by passing in the setting name prefixed with
/// `APP_`.
///
/// Example:
/// ```bash
/// APP_LOG_LEVEL=DEBUG cargo run
/// ````
pub fn config() -> Result<Settings, Error> {
    Figment::from(Serialized::defaults(Settings::default()))
        .merge(Toml::file("Config.toml"))
        .merge(Env::prefixed("APP_"))
        .extract()
}
