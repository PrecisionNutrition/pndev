use structopt::StructOpt;
use clap_verbosity_flag::Verbosity;

//use failure::ResultExt;
use exitfailure::ExitFailure;

use log::{info, warn};

use std::str::FromStr;

mod check;

// Handling command parsing
// taken from https://github.com/Peternator7/strum/blob/master/strum/src/lib.rs#L55
#[derive(Debug)]
enum ParseError {
  VariantNotFound,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        // We could use our macro here, but this way we don't take a dependency on the
        // macros crate.
        match self {
            &ParseError::VariantNotFound => write!(f, "Use --help for info"),
        }
    }
}


// available commands
#[derive(Debug)]
enum Command {
  Check,
  Clone,
  Status,
  //Help
}

impl FromStr for Command {
    type Err = ParseError;
    fn from_str(day: &str) -> Result<Self, Self::Err> {
        match day {
            "check" => Ok(Command::Check),
            "clone" => Ok(Command::Clone),
            "status" => Ok(Command::Status),
            _ => Err(ParseError::VariantNotFound),
        }
    }
}

// CLI definition
#[derive(Debug, StructOpt)]
struct Cli {

  #[structopt(flatten)]
  verbose: Verbosity,
  #[structopt(flatten)]
  log: clap_log_flag::Log,

  /// Available commands: check, clone or status
  command: Command,
}

fn status_command() {
  match check::pn_doctor() {
    Ok(_) => (),
    Err(msg) => msg,
  }
}

fn main() -> Result<(), ExitFailure> {
  let args = Cli::from_args();
  args.log.log_all(args.verbose.log_level())?;

  warn!("LogLevel Warn");
  info!("LogLevel Info");

  match args.command {
    Command::Status => status_command(),
    Command::Check => check::check_all(),
    Command::Clone => (),
  };

  Ok(())
}
