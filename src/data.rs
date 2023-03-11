use config::{Config, ConfigError, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Open {
    pub api: String,
}

impl Open {
    pub fn new() -> Result<Self, ConfigError> {
        let d = shellexpand::tilde("~") + "/.config/msr/openai.toml";
        let s = Config::builder()
            .add_source(File::with_name(&d))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;
        s.try_deserialize()
    }
}
