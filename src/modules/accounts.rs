use crate::command::run_command;
use crate::config;
use crate::config::{Account, Status};
use anyhow::{anyhow, Result};
use log::{debug, trace};
use std::fs;

static GDM_CONFIG: &str = "/etc/gdm/custom.conf";

struct GDMConfig {}

impl GDMConfig {
    fn restart(&self) -> Result<()> {
        match run_command("systemctl restart gdm") {
            Ok(_) => Ok(()),
            Err(error) => Err(anyhow!("Error restart gdm: {}", error)),
        }
    }

    fn read_config(&self) -> Result<String> {
        let contents = fs::read_to_string(GDM_CONFIG)?;

        Ok(contents)
    }

    fn write_config(&self, config: String) -> Result<()> {
        fs::write(GDM_CONFIG, config)?;
        Ok(())
    }

    fn disable_autologin(&self) -> Result<()> {
        let mut config = self.read_config()?;

        config = config.replace("AutomaticLoginEnable=True", "AutomaticLoginEnable=False");

        self.write_config(config)?;

        self.restart()?;

        Ok(())
    }

    fn enable_autologin(&self) -> Result<()> {
        let mut config = self.read_config()?;

        config = config.replace("AutomaticLoginEnable=False", "AutomaticLoginEnable=True");

        self.write_config(config)?;

        self.restart()?;

        Ok(())
    }
}

impl Account {
    async fn lock_computer(&self) -> Result<()> {
        match self.r#type {
            config::Type::Normal => {
                self.lock_account().await?;
            }
            config::Type::GDMAutoLogin => {
                let gdm_config = GDMConfig {};
                gdm_config.disable_autologin()?;
            }
        }

        self.stop_sessions().await?;

        Ok(())
    }

    async fn unlock_computer(&self) -> Result<()> {
        match self.r#type {
            config::Type::Normal => {
                self.unlock_account().await?;
            }
            config::Type::GDMAutoLogin => {
                let gdm_config = GDMConfig {};
                gdm_config.enable_autologin()?;
            }
        }

        Ok(())
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
