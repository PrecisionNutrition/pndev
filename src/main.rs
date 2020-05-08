#![warn(
   clippy::all,
   //clippy::restriction,
   //clippy::pedantic,
   clippy::nursery,
   //clippy::cargo,
)]
#![allow(clippy::non_ascii_literal)]

use clap_verbosity_flag::Verbosity;
use structopt::StructOpt;

use exitfailure::ExitFailure;

use log::{info, warn};

use command::Command;
use std::fmt;

/// Check functions
mod check;

/// Shell functions
mod shell;

/// Git functions
mod git;

/// Command functions
mod command;

/// Self update
mod update;

mod config;

mod opt_log;
mod parse;

#[derive(Debug)]
pub enum ResetType {
    All,
    Docker,
    Local,
}

#[derive(Debug)]
pub struct ParseError {
    msg: &'static str,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::str::FromStr for ResetType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(Self::All),
            "docker" => Ok(Self::Docker),
            "local" => Ok(Self::Local),
            _ => Err(ParseError {
                msg: "options are: all, docker, local",
            }),
        }
    }
}

#[derive(StructOpt, Debug)]
/// Available commands
enum CliCommand {
    #[structopt(name = "doctor")]
    /// diagnose system setup for pndev
    Doctor,

    #[structopt(name = "clone")]
    /// clone one or all the pn apps into ~/DEV/PN
    Clone {
        #[structopt(short = "a", long = "all")]
        /// clones the main pn apps (es, fitpro, student..)
        all: bool,

        #[structopt(name = "name")]
        /// name of the repository
        name: Option<String>,
    },

    #[structopt(name = "review")]
    /// checkout a PR in one or all APP repos (not including addons)
    Review {
        #[structopt(name = "pr")]
        /// branch name (JIRA ticket ID)
        pr: Option<String>,

        #[structopt(long = "name")]
        /// name of the repository
        name: Option<String>,
    },

    #[structopt(name = "prepare")]
    /// prepares the db for eternal-sledgehammer
    Prepare {
        #[structopt(short = "q", long = "quick")]
        /// Do not use a remote snapshot, just bootstrap, no program data will be restored
        quick: bool,
    },

    #[structopt(name = "start")]
    /// start docker and ember s or rails - depends on application
    Start {
        #[structopt(short = "d", long = "only-docker")]
        /// do not attempt to start also rails or ember apps
        docker: bool,
    },

    #[structopt(name = "stop")]
    /// DEPRECATED use down instead
    Stop,

    #[structopt(name = "up")]
    /// runs docker-compose up on pndev docker services, same as start -d
    Up,

    #[structopt(name = "down")]
    /// runs docker-compose down on pndev docker services
    Down,

    #[structopt(name = "shell")]
    /// start a nix-shell in the current application
    Shell,

    #[structopt(name = "ps")]
    /// print docker status
    Ps,

    #[structopt(name = "reset")]
    /// Nukes the nix-shell config (use when ruby/node version changes)
    Reset {
        /// all, docker, local , docker forces a rebuild, local deletes local dependencies
        #[structopt(name = "reset type")]
        docker_or_local: ResetType,
    },

    #[structopt(name = "update")]
    /// attempts to update pndev itself to the latest released version
    Update,

    #[structopt(name = "rebuild")]
    /// rebuild docker containers after downloading new config
    Rebuild,
}

// CLI definition
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    verbose: Verbosity,
    #[structopt(flatten)]
    log: opt_log::Log,

    #[structopt(subcommand)]
    command: CliCommand,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    args.log.log_all(args.verbose.log_level())?;

    warn!("LogLevel Warn");
    info!("LogLevel Info");

    let command_result = match args.command {
        CliCommand::Prepare { quick } => Command::prepare(quick),
        CliCommand::Shell => Command::shell(),
        CliCommand::Up => Command::up(),
        CliCommand::Start { docker } => Command::start(docker),
        CliCommand::Down => Command::down(),
        CliCommand::Ps => Command::ps(),
        CliCommand::Reset { docker_or_local } => Command::reset(docker_or_local),
        CliCommand::Doctor => check::doctor(),
        CliCommand::Clone { name, all } => Command::clone(name, all),
        CliCommand::Review { pr, name } => Command::review(pr, name),
        CliCommand::Update => update::run(),
        CliCommand::Stop => {
            println!("stop is DEPRECATED, use `pndev down` instead");
            Command::down()
        }
        CliCommand::Rebuild => {
            println!("rebuild is DEPRECATED, use `pndev reset docker` instead");
            Command::reset(ResetType::Docker)
        }
    };

    Ok(command_result?)
}
