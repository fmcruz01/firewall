use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "RSCAN", version = "1.0", about = "RSCAN v1.0 -- LAN Scanning Tool", long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Command,
    /// Path to the output file. If file does not exist, one will be created
    #[arg(short, long)]
    output: Option<String>,

}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Scans LAN to discover devices
    Discover {
        #[arg(short, long, default_value_t = String::from("default"))]
        interface: String,
    },
    /// Lists all information about device with provided id
    Fingerprint {
        /// ip of the device to fingerprint
        ip: String,
    },
}

pub fn parse() -> Command {
    Args::parse().command
}
