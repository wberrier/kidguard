use confy;
use log::{error, info};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AccountStatus {
    Locked,
    Unlocked,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub username: String,
    pub account_status: AccountStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub users: Vec<User>,
}

/// `Config` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self { users: Vec::new() }
    }
}

// TODO: automatic way to store in /etc?
const _CONFIG_NAME: &str = "kidguard";
const CONFIG_PATH: &str = "/etc/kidguard/kidguard.yml";

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let mut config = Config::default();
    if let Ok(conf) = confy::load_path(CONFIG_PATH) {
        info!("Successfully loaded config");
        config = conf;
    }

    if let Err(error) = confy::store_path(CONFIG_PATH, &config) {
        error!("Config error: {}", error);
    }

    config
});
