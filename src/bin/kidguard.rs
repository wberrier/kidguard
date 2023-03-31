use anyhow::Result;
use log::{error, info, trace};
use std::time::Duration;
use tokio::signal::unix::{signal, SignalKind};

use kidguard::modules::accounts;

struct KGState {
    shutdown: bool,
}

impl KGState {
    fn new() -> Self {
        Self { shutdown: false }
    }
}

async fn start() -> Result<()> {
    trace!("Inside function");
    let mut state = KGState::new();
    // Handle ctrl-c
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    tokio::task::spawn(async move {
        let mut timer = tokio::time::interval(Duration::from_secs(10));

        loop {
            if state.shutdown {
                info!("Shutting down...");
                break;
            }

            tokio::select! {
                _ = timer.tick() => {
                    if accounts::configure_accounts().await.is_err() {
                        error!("Error configuring the accounts");
                    }
                }
                Some(_) = sigint.recv() => {
                    info!("Got sigint");
                    _ = accounts::configure_accounts().await;
                    state.shutdown = true;
                }
                Some(_) = sigterm.recv() => {
                    info!("Got terminate");
                    _ = accounts::configure_accounts().await;
                    state.shutdown = true;
                }
            }
        }
    })
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    start().await?;

    Ok(())
}
