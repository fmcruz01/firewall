use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "RSCAN", version = "1.0", about = "RSCAN v1.0 -- LAN Scanning Tool", long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
    /// Path to the output file. If file does not exist, one will be created
    #[arg(short, long)]
    output: Option<String>,

}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Scans LAN network to discover devices
    Discover {
        #[arg(short, long, default_value_t = String::from("en0"))]
        interface: String,
    },
    /// Lists all information about device with provided id
    Fingerprint {
        /// id of the device to fingerprint
        id: String,
    },
}

fn main() {
    let args = Args::parse();
}
