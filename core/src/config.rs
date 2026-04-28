use crate::errors::{Error, ErrorType};

pub enum Mode {
    TUI,
    SNIFF,
}

pub struct Cli {
    mode: Mode,
    verbose: bool,
    file: Option<String>,
}

impl Cli {
    pub fn parse() -> Result<Self, Error> {
        let mut args = std::env::args().skip(1);
        let mut mode = Mode::TUI;
        let mut verbose = false;
        let mut file = None;
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-s" | "--sniff" => {
                    mode = Mode::SNIFF;
                }
                "-v" | "--verbose" => {
                    verbose = true;
                }
                "-f" | "--file" => {
                    file = args.next();
                    if file == None {
                        return Err(Error::err(
                            ErrorType::ArgParseError,
                            "File path is required with --file option.".to_string(),
                        ));
                    }
                }
                "-V" | "--version" => {
                    println!("{}", env!("CARGO_PKG_VERSION"));
                }
                "-h" | "--help" => {
                    help();
                }
                _ => {
                    usage_display();
                }
            }
        }
        Ok(Cli {
            mode,
            verbose,
            file,
        })
    }
}

pub fn usage_display() {
    println!(
        "usage: rscan [OPTIONS]

        Options:
        -t, --tui           Terminal User Interface mode. This is the default mode.
        -s, --sniff         Passive mode. Displaying packets metadata.
        -v, --verbose       Verbose output logging.
        -f, --file <FILE>   Write packet data to the specified file.
        -h, --help          Print help.
        -V, --version       Print version.\n"
    );
}

fn help() {
    println!("RSCAN v0.1.0 — Network Recon CLI\n");
    usage_display();
}

fn sniff_intro() {
    let banner = r#"
 ____  ____   ____    _    _   _
|  _ \/ ___| / ___|  / \  | \ | |
| |_) \___ \| |     / _ \ |  \| |
|  _ < ___) | |___ / ___ \| |\  |
|_| \_\____/ \____/_/   \_\_| \_|
"#;
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear terminal and position cursor at 1,1
    println!("\x1b[36m{}\x1b[0m", banner); // cyan
    println!("RSCAN v0.1.0 — Network Recon CLI\n");
}
