use crate::command::run_command;
use crate::config;
use crate::config::{Account, Status};
use anyhow::{anyhow, Result};
use log::{debug, trace};

impl Account {
    async fn lock_computer(&self) -> Result<()> {
        self.lock_account().await?;
        self.stop_sessions().await
    }

    async fn unlock_computer(&self) -> Result<()> {
        self.unlock_account().await
    }

    async fn lock_account(&self) -> Result<()> {
        trace!("Locking account: {}", self.username);
        match run_command(format!("passwd -l '{}'", self.username).as_str()) {
            Ok(_) => Ok(()),
            Err(error) => Err(anyhow!("Error locking account: {}", error)),
        }
    }
    async fn unlock_account(&self) -> Result<()> {
        trace!("Unlocking account: {}", self.username);
        match run_command(format!("passwd -u '{}'", self.username).as_str()) {
            Ok(_) => Ok(()),
            Err(error) => Err(anyhow!("Error unlocking account: {}", error)),
        }
    }

    async fn stop_sessions(&self) -> Result<()> {
        trace!("Stopping sessions: {}", self.username);
        match run_command(format!("loginctl terminate-user '{}'", self.username).as_str()) {
            Ok(_) => Ok(()),
            Err(error) => {
                debug!("Stopping sessions failed for {}: {}", self.username, error);
                Ok(())
            }
        }
    }
}

async fn configure_account(account: &Account) -> Result<()> {
    match account.status {
        Status::Locked => account.lock_computer().await,
        Status::Unlocked => account.unlock_computer().await,
    }
}

pub async fn configure_accounts() -> Result<()> {
    let conf = &config::CONFIG;

    for account in &conf.accounts {
        debug!("Configuring user: {}", account.username);
        configure_account(account).await?;
    }

    Ok(())
}
