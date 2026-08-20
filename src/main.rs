mod cli;
mod discover;
mod fingerprint;
mod models;

use anyhow::Result;

fn main() -> Result<()> {
    let cmd = cli::parse();
    match cmd {
        cli::Command::Discover { interface } => discover::discover(),
        cli::Command::Fingerprint { id } => {}
    }
    Ok(())
}
