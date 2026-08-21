mod cli;
mod discover;
mod fingerprint;
mod models;

use anyhow::{Error, Result};

fn main() -> Result<()> {
    let cmd = cli::parse();
    match cmd {
        cli::Command::Discover { interface } => {
            if let Err(error) = discover::scan_network() {
                print_error(&error);
                std::process::exit(1);
            }
        }
        cli::Command::Fingerprint { id } => {
            if let Err(error) = fingerprint::scan_device() {
                print_error(&error);
                std::process::exit(1);
            }
        }
    }
    Ok(())
}

fn print_error(err: &Error) {
    eprintln!("Error: {}", err);

    eprintln!();
    eprintln!("Caused by:");

    for (i, cause) in err.chain().skip(1).enumerate() {
        eprintln!("    {i}: {cause}");
    }
}
