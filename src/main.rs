use structopt::StructOpt;
use clap_verbosity_flag::Verbosity;

use failure::Error;
use exitfailure::ExitFailure;

use log::{info, warn, trace};

use std::str::FromStr;

mod check;
mod shell;

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
  Shell,
  Start,
  Stop,
  //Help
}

impl FromStr for Command {
    type Err = ParseError;
    fn from_str(day: &str) -> Result<Self, Self::Err> {
        match day {
            "check" => Ok(Command::Check),
            "clone" => Ok(Command::Clone),
            "status" => Ok(Command::Status),
            "shell" => Ok(Command::Shell),
            "start" => Ok(Command::Start),
            "stop" => Ok(Command::Stop),
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

  /// Available commands: check, clone, status, shell, start, stop
  command: Command,
}

fn status_command() -> Result<(), Error> {
  check::pn_doctor()?;
  // TODO decide if we need this command

  Ok(())
}

fn shell_command() -> Result<(), Error> {
  check::pn_doctor()?;

  trace!("shell started ");

  shell::nix_shell()?;

  trace!("shell closed");

  Ok(())
}

fn start_command() -> Result<(), Error> {
  check::pn_doctor()?;

  trace!("start command");

  shell::docker_up()?;

  shell::forego_start()?;

  trace!("start command done");

  Ok(())
}

fn stop_command() -> Result<(), Error> {
  check::pn_doctor()?;

  trace!("stop command");

  shell::docker_down()?;

  trace!("stop command done");

  Ok(())
}

fn main() -> Result<(), ExitFailure> {
  let args = Cli::from_args();
  args.log.log_all(args.verbose.log_level())?;

  warn!("LogLevel Warn");
  info!("LogLevel Info");

  let command_result = match args.command {
    Command::Status => status_command(),
    Command::Shell => shell_command(),
    Command::Start => start_command(),
    Command::Stop => stop_command(),
    Command::Check => check::check_all(),
    Command::Clone => Ok(()),
  };

  Ok(command_result?)
}
