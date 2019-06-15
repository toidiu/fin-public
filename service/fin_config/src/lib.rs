#[macro_use]
extern crate serde_derive;

use config::{Config, ConfigError, Environment, File};
use std::env;

#[derive(Debug, Deserialize)]
pub struct App {
    pub port: u16,
    pub paseto_token: String,
    pub paseto_timeout_min: i64,
    pub cors_origin: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
    pub pool_size: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IexConfig {
    pub debug: bool,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct FinConfig {
    pub app: App,
    // debug: bool,
    pub database: Database,
    pub iex: IexConfig,
}

impl FinConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("fin_config/default"))?;

        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        let env = env::var("RUN_MODE").unwrap_or("development".into());
        s.merge(
            File::with_name(&format!("fin_config/{}", env)).required(false),
        )?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name("fin_config/local").required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("app"))?;

        // You may also programmatically change settings
        // s.set("database.url", "postgres://")?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}
