use core::{config::{Cli, usage_display}};
use std::process::exit;

fn main() {
    if let Err(err) = Cli::parse() {
        eprintln!("{}", err);
        usage_display();
        exit(1);
    }
}
