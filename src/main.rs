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

/// Check functions
mod check;

/// Shell functions
mod shell;

/// Git functions
mod git;

/// Command functions
mod command;
use command::Command;

/// Self update
mod update;

mod config;

#[derive(StructOpt, Debug)]
/// Available commands
enum CliCommand {
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

    #[structopt(name = "doctor")]
    /// diagnose system setup for pndev
    Doctor,

    #[structopt(name = "prepare")]
    /// run optional setup steps (i.e db setup)
    Prepare,

    #[structopt(name = "shell")]
    /// start a nix-shell in the current application
    Shell,

    #[structopt(name = "start")]
    /// start docker and ember s or rails - depends on application
    Start {
        #[structopt(short = "d", long = "only-docker")]
        /// do not attempt to start also rails or ember apps
        docker: bool,
    },

    #[structopt(name = "stop")]
    /// stop docker
    Stop,

    #[structopt(name = "rebuild")]
    /// rebuild docker containers after downloading new config
    Rebuild,

    #[structopt(name = "reset")]
    /// Nukes the nix-shell config (use when ruby/node version changes)
    Reset,

    #[structopt(name = "ps")]
    /// print docker status
    Ps,

    #[structopt(name = "update")]
    /// attempts to update pndev itself to the latest released version
    Update,
}

// CLI definition
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    verbose: Verbosity,
    #[structopt(flatten)]
    log: clap_log_flag::Log,

    #[structopt(subcommand)]
    command: CliCommand,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    args.log.log_all(args.verbose.log_level())?;

    warn!("LogLevel Warn");
    info!("LogLevel Info");

    config::Config::new();

    let command_result = match args.command {
        CliCommand::Prepare => Command::prepare(),
        CliCommand::Shell => Command::shell(),
        CliCommand::Start { docker } => Command::start(docker),
        CliCommand::Stop => Command::stop(),
        CliCommand::Ps => Command::ps(),
        CliCommand::Rebuild => Command::rebuild(),
        CliCommand::Reset => Command::reset(),
        CliCommand::Doctor => check::doctor(),
        CliCommand::Clone { name, all } => Command::clone(name, all),
        CliCommand::Update => update::run(),
    };

    Ok(command_result?)
}
