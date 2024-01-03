use confy;
use log::{error, info};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Status {
    Locked,
    Unlocked,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Type {
    Normal, // TODO: bad name?
    GDMAutoLogin,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    pub username: String,
    pub status: Status,
    pub r#type: Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub accounts: Vec<Account>,
}

/// `Config` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            accounts: Vec::new(),
        }
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
