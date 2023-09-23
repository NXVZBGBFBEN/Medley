use confy::ConfyError;
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
struct RawConfig {
    engine: String,
}

impl Default for RawConfig {
    fn default() -> Self {
        Self {
            engine: String::from("internal"),
        }
    }
}

#[derive(Default)]
pub struct ParsedConfig {
    pub engine: Engine,
}

#[derive(Default)]
pub enum Engine {
    #[default]
    Internal,
    Maxima,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("failed to load config file")]
    FailedLoading(#[from] ConfyError),
    #[error("invalid value {found}")]
    InvalidValue { found: String },
}

impl ConfigError {
    pub fn kind(&self) -> String {
        String::from("config file error")
    }
}

pub fn load_config() -> Result<ParsedConfig, ConfigError> {
    let mut config = ParsedConfig::default();
    /* 設定値読み取り */
    let loaded_config = confy::load::<RawConfig>("medley", "config")?;
    /* 値の検証 */
    match loaded_config.engine.as_str() {
        "internal" => config.engine = Engine::Internal,
        "maxima" => config.engine = Engine::Maxima,
        unexpected_value => {
            return Err(ConfigError::InvalidValue {
                found: String::from(unexpected_value),
            })
        }
    };
    Ok(config)
}
