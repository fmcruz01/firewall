// Implement CLI commands
//
// Handle commands like:
//   load rules
//   validate config
//   show status
// Call into config + IPC layers
use std::result::Result;
use fw_user::{start_processing, RuntimeError};

pub enum Command {
    SNIFF { interface: String, verbose: bool },
}

impl Command {
    pub fn execute(&self) -> Result<(), RuntimeError>{
        match self {
            Command::SNIFF { interface: _, verbose: v } => {
                start_processing(*v)
            }
        }
    }
}

pub fn parse_command(cmd: &str) -> Result<Command, &'static str> {
    match cmd {
        "sniff" => {
            println!("Sniff command received.");
            Ok(Command::SNIFF {
                interface: "eth0".to_string(),
                verbose: true,
            })
        }
        _ => Err("unsupported command"),
    }
}
