//! Not sure which process execution I like, so just abstract one here
//! Also not sure about which async one to use as well...

//! Maybe for this project we don't need async process yet...

use anyhow::{anyhow, Result};
use runcmd::RunCmd;

// for now, only return result
pub fn run_command(command: &str) -> Result<()> {
    match RunCmd::new(command).execute().exitcode {
        0 => Ok(()),
        _ => Err(anyhow!("Error running command: {}", command)),
    }
}
