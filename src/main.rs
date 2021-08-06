#![warn(
   clippy::all,
   clippy::nursery,
   //clippy::restriction,
   //clippy::pedantic,
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

/// Configuration
mod config;

/// Utils
mod opt_log;
mod parse;

#[derive(Debug)]
pub enum ResetType {
    Docker,
    Deps,
    Scratch,
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
            "scratch" => Ok(Self::Scratch),
            "docker" => Ok(Self::Docker),
            "deps" => Ok(Self::Deps),
            _ => Err(ParseError {
                msg: "options are: docker, deps, scratch",
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
        /// Remote snapshot with no client data, no program data will be restored
        quick: bool,
        #[structopt(short = "b", long = "big")]
        /// User a remote snapshot with some anonymized client data, then bootstrap
        big: bool,
    },

    #[structopt(name = "start")]
    /// start docker and ember s or rails - depends on application
    Start {
        #[structopt(short = "d", long = "only-docker")]
        /// do not attempt to start also rails or ember apps
        docker: bool,
    },

    #[structopt(name = "up")]
    /// runs docker-compose up on pndev docker services, same as start -d
    Up,

    #[structopt(name = "down")]
    /// runs docker-compose down on pndev docker services
    Down,

    #[structopt(name = "shell")]
    /// start a nix-shell in the current application
    Shell {
        /// optional command to run in the shell
        command: Vec<String>,
    },

    #[structopt(name = "sh")]
    /// alias to pndev shell
    Sh {
        /// optional command to run in the shell
        command: Vec<String>,
    },

    #[structopt(name = "ps")]
    /// print docker status
    Ps,

    #[structopt(name = "reset")]
    /// Nukes the nix-shell config (use when ruby/node version changes)
    Reset {
        /// deps, docker, scratch:  deps deletes local dependencies, docker updates the docker config, scratch wipes everything, including git changes
        #[structopt(name = "reset type")]
        reset_type: ResetType,
    },

    #[structopt(name = "update")]
    /// attempts to update pndev itself to the latest released version
    Update,

    #[structopt(name = "rebuild")]
    /// rebuild docker containers after downloading new config
    Rebuild,

    #[structopt(name = "gh")]
    /// opens the corresponding repo on github if available
    Gh,

    #[structopt(name = "run")]
    /// run a command by name
    Run {
        #[structopt(name = "name")]
        /// name of the command in pndev.toml
        name: Option<String>,

        /// optional arguments for the run command
        arguments: Vec<String>,
    },

    #[structopt(external_subcommand)]
    Other(Vec<String>),
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
        CliCommand::Prepare { quick, big } => {
            if quick {
                println!("pndev prepare -q is DEPRECATED as it is the default now, use --big for the old prepare");
            }
            Command::prepare(big)
        }
        CliCommand::Shell { command } => Command::shell(command),
        CliCommand::Sh { command } => Command::shell(command),
        CliCommand::Up => Command::up(),
        CliCommand::Start { docker } => Command::start(docker),
        CliCommand::Down => Command::down(),
        CliCommand::Ps => Command::ps(),
        CliCommand::Reset { reset_type } => Command::reset(reset_type),
        CliCommand::Doctor => check::doctor(),
        CliCommand::Clone { name, all } => Command::clone(name, all),
        CliCommand::Review { pr, name } => Command::review(pr, name),
        CliCommand::Update => update::run(),
        CliCommand::Rebuild => {
            println!("rebuild is DEPRECATED, use `pndev reset docker` instead");
            Command::reset(ResetType::Docker)
        }
        CliCommand::Gh => Command::gh(),
        CliCommand::Run { name, arguments } => Command::run(name, arguments),
        CliCommand::Other(list) => {
            let name = &list[0];
            let arguments = &list[1..];
            Command::run(Some(name.into()), Vec::from(arguments))
        }
    };

    Ok(command_result?)
}
