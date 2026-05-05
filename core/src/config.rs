use crate::errors::{Error, ErrorType};

#[derive(Debug, PartialEq)]
pub enum Mode {
    TUI,
    PASSIVE,
    ACTIVE,
    VERSION,
    HELP,
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub mode: Mode,
    pub verbose: bool,
}

impl Config {
    fn default() -> Self {
        Self {
            mode: Mode::TUI,
            verbose: false,
        }
    }

    fn set_mode(&mut self, mode_set: &mut bool, new_mode: Mode) -> Result<(), Error> {
        if *mode_set {
            return Err(Error::err(
                ErrorType::ArgParseError,
                "Multiple modes specified, only one is supported".to_string(),
            ));
        }
        self.mode = new_mode;
        *mode_set = true;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum Cli {
    Config(Config),
    Help,
    Version,
}

impl Cli {
    pub fn parse<I>(args: I) -> Result<Self, Error>
    where
        I: Iterator<Item = String>,
    {
        let mut config = Config::default();
        let mut mode_set = false;
        for arg in args.skip(1) {
            if arg.starts_with("--") {
                match arg.as_str() {
                    "--version" => return Ok(Cli::Version),
                    "--help" => return Ok(Cli::Help),
                    "--tui" => config.set_mode(&mut mode_set, Mode::TUI)?,
                    "--passive" => config.set_mode(&mut mode_set, Mode::PASSIVE)?,
                    "--active" => config.set_mode(&mut mode_set, Mode::ACTIVE)?,
                    "--verbose" => config.verbose = true,
                    _ => {
                        return Err(Error::err(
                            ErrorType::ArgParseError,
                            "Unknown argument.".to_string(),
                        ));
                    }
                }
            } else if arg.starts_with("-") {
                for ch in arg.chars().skip(1) {
                    match ch {
                        'V' => return Ok(Cli::Version),
                        'h' => return Ok(Cli::Help),
                        't' => config.set_mode(&mut mode_set, Mode::TUI)?,
                        'p' => config.set_mode(&mut mode_set, Mode::PASSIVE)?,
                        'a' => config.set_mode(&mut mode_set, Mode::ACTIVE)?,
                        'v' => config.verbose = true,
                        _ => {
                            return Err(Error::err(
                                ErrorType::ArgParseError,
                                "Unknown argument.".to_string(),
                            ));
                        }
                    }
                }
            } else {
            }
        }
        Ok(Cli::Config(config))
    }
}

pub fn usage_display() {
    println!(
        "usage: rscan [MODE] [OPTIONS]

        Arguments:
        -t, --tui           Terminal User Interface mode. (Default mode).
        -p, --passive       Passive mode. (Displaying packets metadata).
        -a, --active        Active mode (Host/port discovery).


        Options:
        -v, --verbose       Verbose output logging.
        -h, --help          Print help.
        -V, --version       Print version.

        Notes:
        If no mode is specified, --tui is assumed.\n"
    );
}

pub fn help() {
    version();
    usage_display();
}

pub fn version() {
    println!("RSCAN {} — Network Recon CLI\n", env!("CARGO_PKG_VERSION"));
}

pub fn intro_banner() {
    let banner = r#"
 ____  ____   ____    _    _   _
|  _ \/ ___| / ___|  / \  | \ | |
| |_) \___ \| |     / _ \ |  \| |
|  _ < ___) | |___ / ___ \| |\  |
|_| \_\____/ \____/_/   \_\_| \_|
"#;
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear terminal and position cursor at 1,1
    println!("\x1b[36m{}\x1b[0m", banner); // cyan
    println!("RSCAN {} — Network Recon CLI\n", env!("CARGO_PKG_VERSION"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_help_success() {
        let valid_args: Vec<Vec<&str>> = vec![
            vec!["-h"],
            vec!["--help"],
            vec!["-h", "unknown_cmd"],
            vec!["--help", "unknown_cmd"],
        ];
        for arg in valid_args {
            let mut args: Vec<String> = arg.iter().map(|a| a.to_string()).collect();
            let mut input: Vec<String> = vec!["rscan".to_string()];
            input.append(&mut args);
            assert_eq!(Cli::parse(input.into_iter()), Ok(Cli::Help));
        }
    }

    #[test]
    fn test_parse_version_success() {
        let valid_args: Vec<Vec<&str>> = vec![
            vec!["-V"],
            vec!["--version"],
            vec!["-V", "unknown_cmd"],
            vec!["--version", "unknown_cmd"],
        ];
        for arg in valid_args {
            let mut args: Vec<String> = arg.iter().map(|a| a.to_string()).collect();
            let mut input: Vec<String> = vec!["rscan".to_string()];
            input.append(&mut args);
            assert_eq!(Cli::parse(input.into_iter()), Ok(Cli::Version));
        }
    }

    #[test]
    fn test_parse_active_success() {
        let valid_args_no_verbose: Vec<Vec<&str>> = vec![vec!["-a"], vec!["--active"]];
        let valid_args_verbose: Vec<Vec<&str>> = vec![
            vec!["-av"],
            vec!["-va"],
            vec!["--verbose", "--active"],
            vec!["--active", "--verbose"],
        ];
        for arg in valid_args_verbose {
            let mut args: Vec<String> = arg.iter().map(|a| a.to_string()).collect();
            let mut input: Vec<String> = vec!["rscan".to_string()];
            input.append(&mut args);
            assert_eq!(
                Cli::parse(input.into_iter()),
                Ok(Cli::Config(Config {
                    mode: Mode::ACTIVE,
                    verbose: true,
                }))
            );
        }

        for arg in valid_args_no_verbose {
            let mut args: Vec<String> = arg.iter().map(|a| a.to_string()).collect();
            let mut input: Vec<String> = vec!["rscan".to_string()];
            input.append(&mut args);
            assert_eq!(
                Cli::parse(input.into_iter()),
                Ok(Cli::Config(Config {
                    mode: Mode::ACTIVE,
                    verbose: false,
                }))
            );
        }
    }

    #[test]
    fn test_parse_tui_success() {
        let valid_args_no_verbose: Vec<Vec<&str>> = vec![vec!["-t"], vec!["--tui"], vec![]];
        let valid_args_verbose: Vec<Vec<&str>> = vec![
            vec!["-tv"],
            vec!["-vt"],
            vec!["--verbose", "--tui"],
            vec!["--tui", "--verbose"],
        ];
        for arg in valid_args_verbose {
            let mut args: Vec<String> = arg.iter().map(|a| a.to_string()).collect();
            let mut input: Vec<String> = vec!["rscan".to_string()];
            input.append(&mut args);
            assert_eq!(
                Cli::parse(input.into_iter()),
                Ok(Cli::Config(Config {
                    mode: Mode::TUI,
                    verbose: true,
                }))
            );
        }

        for arg in valid_args_no_verbose {
            let mut args: Vec<String> = arg.iter().map(|a| a.to_string()).collect();
            let mut input: Vec<String> = vec!["rscan".to_string()];
            input.append(&mut args);
            assert_eq!(
                Cli::parse(input.into_iter()),
                Ok(Cli::Config(Config {
                    mode: Mode::TUI,
                    verbose: false,
                }))
            );
        }
    }

    #[test]
    fn test_parse_passive_success() {
        let valid_args_no_verbose: Vec<Vec<&str>> = vec![vec!["-p"], vec!["--passive"]];
        let valid_args_verbose: Vec<Vec<&str>> = vec![
            vec!["-pv"],
            vec!["-vp"],
            vec!["--verbose", "--passive"],
            vec!["--passive", "--verbose"],
        ];
        for arg in valid_args_verbose {
            let mut args: Vec<String> = arg.iter().map(|a| a.to_string()).collect();
            let mut input: Vec<String> = vec!["rscan".to_string()];
            input.append(&mut args);
            assert_eq!(
                Cli::parse(input.into_iter()),
                Ok(Cli::Config(Config {
                    mode: Mode::PASSIVE,
                    verbose: true,
                }))
            );
        }

        for arg in valid_args_no_verbose {
            let mut args: Vec<String> = arg.iter().map(|a| a.to_string()).collect();
            let mut input: Vec<String> = vec!["rscan".to_string()];
            input.append(&mut args);
            assert_eq!(
                Cli::parse(input.into_iter()),
                Ok(Cli::Config(Config {
                    mode: Mode::PASSIVE,
                    verbose: false,
                }))
            );
        }
    }

    #[test]
    fn test_parse_fail() {
        let invalid_args: Vec<Vec<&str>> = vec![
            vec!["--unknown"],                        // unknown arg
            vec!["-u"],                               // unknown arg
            vec!["-avt"],                             // multiple modes
            vec!["--verbose", "--passsive", "--tui"], //multiple modes
            vec!["--tui", "--unknown"],               // multiple args with unknown
            vec!["-tvu"],                             // multiple args with unknown
        ];
        for arg in invalid_args {
            let mut args: Vec<String> = arg.iter().map(|a| a.to_string()).collect();
            let mut input: Vec<String> = vec!["rscan".to_string()];
            input.append(&mut args);
            let res = Cli::parse(input.into_iter()).err().unwrap().err_type;
            assert_eq!(res, ErrorType::ArgParseError);
        }
    }
}
