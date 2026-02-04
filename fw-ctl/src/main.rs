// CLI entry point
//
// Parse user commands
// Dispatch subommands
// Handle errors cleanly
mod cli;
use cli::command::parse_command;
use std::env::args;
use std::process::exit;

pub fn main() {
    let args: Vec<String> = args().collect();
    match args.len() {
        1 => {
            help();
            exit(1);
        }
        2 => {
            match parse_command(&args[1]) {
                Ok(command) => {
                    command.execute();
                }
                Err(error) => {
                    println!("{error}");
                }
            }
        }
        _ => {
            help();
            exit(0);
        }
    }
}

fn help() {
    println!("usage:
fw-ctl sniff [ -v ]
    Start firewall. Use -v for verbose mode.
");
}
