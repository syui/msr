use config::{Config, ConfigError, File};
use serde_derive::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Data {
    pub base: Cow<'static, str>,
    pub token: Cow<'static, str>,
    pub client_id: Cow<'static, str>,
    pub client_secret: Cow<'static, str>,
    pub redirect: Cow<'static, str>,
    pub misskey_base: url::Url,
    pub misskey_api: url::Url,
    pub misskey_stream: url::Url,
    pub misskey_token: Cow<'static, str>,
}

impl Data {
    pub fn new() -> Result<Self, ConfigError> {
        let d = shellexpand::tilde("~") + "/.config/msr/config.toml";
        let s = Config::builder()
            .add_source(File::with_name(&d))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;
        s.try_deserialize()
    }
}

