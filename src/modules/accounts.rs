use crate::command::run_command;
use crate::config;
use crate::config::{AccountStatus, User};
use anyhow::{anyhow, Result};
use log::debug;

struct Account {
    username: String,
}

impl Account {
    fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
        }
    }

    async fn lock_computer(&self) -> Result<()> {
        self.lock_account().await?;
        self.stop_sessions().await
    }

    async fn unlock_computer(&self) -> Result<()> {
        self.unlock_account().await
    }

    async fn lock_account(&self) -> Result<()> {
        match run_command(format!("passwd -l '{}'", self.username).as_str()) {
            Ok(_) => Ok(()),
            Err(error) => Err(anyhow!("Error locking account: {}", error)),
        }
    }
    async fn unlock_account(&self) -> Result<()> {
        match run_command(format!("passwd -u '{}'", self.username).as_str()) {
            Ok(_) => Ok(()),
            Err(error) => Err(anyhow!("Error unlocking account: {}", error)),
        }
    }

    async fn stop_sessions(&self) -> Result<()> {
        match run_command(format!("loginctl terminate-user '{}'", self.username).as_str()) {
            Ok(_) => Ok(()),
            Err(error) => Err(anyhow!("Error stopping sessions: {}", error)),
        }
    }
}

async fn configure_account(user: &User) -> Result<()> {
    let account = Account::new(&user.username);
    match user.account_status {
        AccountStatus::Locked => account.lock_computer().await,
        AccountStatus::Unlocked => account.unlock_computer().await,
    }
}

pub async fn configure_accounts() -> Result<()> {
    let conf = &config::CONFIG;

    for user in &conf.users {
        debug!("Configuring user: {}", user.username);
        configure_account(user).await?;
    }

    Ok(())
}
