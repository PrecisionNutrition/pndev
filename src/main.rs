#![warn(
   clippy::all,
   //clippy::restriction,
   clippy::pedantic,
   clippy::nursery,
   //clippy::cargo,
)]
#![allow(clippy::non_ascii_literal)]

use clap_verbosity_flag::Verbosity;
use structopt::StructOpt;

use exitfailure::ExitFailure;
use failure::bail;
use failure::Error;

use log::{info, trace, warn};

use dirs::home_dir;
use std::path::Path;

use self_update;

// unix specific extensions for chmod
use std::os::unix::fs::PermissionsExt;

/// Check functions
mod check;

/// Shell functions
mod shell;

/// Git functions
mod git;

#[derive(StructOpt, Debug)]
/// Available commands
enum Command {
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

    #[structopt(name = "ps")]
    /// print docker status
    Ps,

    #[structopt(name = "update")]
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
    command: Command,
}

fn shell_command() -> Result<(), Error> {
    check::all()?;

    trace!("shell started ");

    shell::nix()?;

    trace!("shell closed");

    Ok(())
}

fn start_command(docker_only: bool) -> Result<(), Error> {
    check::all()?;

    trace!("start command");

    shell::docker_up()?;

    if docker_only {
        info!("Starting only docker services");
        return Ok(());
    }

    if Path::new("Gemfile.lock").exists() {
        shell::forego_start()?;
    } else if Path::new("yarn.lock").exists() {
        shell::ember_start()?;
    } else {
        bail!("No bundle or ruby config found")
    }

    trace!("start command done");

    Ok(())
}

fn stop_command() -> Result<(), Error> {
    check::all()?;

    trace!("stop command");

    shell::docker_down()?;

    trace!("stop command done");

    Ok(())
}

fn ps_command() -> Result<(), Error> {
    check::all()?;

    trace!("ps command");

    println!("Docker ps output:");

    shell::docker_ps()?;

    trace!("ps command done");

    Ok(())
}

fn prepare_command() -> Result<(), Error> {
    check::all()?;

    trace!("anonymize command");
    let mut path = home_dir().unwrap();
    path.push(".pn_anonymize_creds");

    if !path.exists() {
        bail!("Please create ~/.pn_anonymize_creds")
    }

    shell::docker_up()?;

    if Path::new("Gemfile.lock").exists() {
        shell::npm_rebuild_deps()?;

        shell::rails_migrate()?;

        shell::rails_anonymize()?;

        shell::rails_bootstrap()?;
    } else {
        bail!("No Gemfile found, are you in the right directory?")
    }

    Ok(())
}

fn clone_command(name: Option<String>, all: bool) -> Result<(), Error> {
    check::all()?;

    trace!("clone command");

    let apps = [
        "eternal-sledgehammer",
        "es-student",
        "fitpro",
        "es-certification",
        "payment-next",
    ];

    if all {
        for &app in &apps {
            println!("Cloning {}", app);
            git::clone(app)?;
        }
    } else {
        match name {
            Some(name) => {
                println!("Cloning {}", name);
                git::clone(&name[..])?;
            }
            None => bail!("Please specify an app name or --all"),
        }
    };

    info!("Clone completed");

    Ok(())
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    args.log.log_all(args.verbose.log_level())?;

    warn!("LogLevel Warn");
    info!("LogLevel Info");

    let command_result = match args.command {
        Command::Prepare => prepare_command(),
        Command::Shell => shell_command(),
        Command::Start { docker } => start_command(docker),
        Command::Stop => stop_command(),
        Command::Ps => ps_command(),
        Command::Doctor => check::doctor(),
        Command::Clone { name, all } => clone_command(name, all),
        Command::Update => update(),
    };

    Ok(command_result?)
}

// see https://github.com/jaemk/self_update#usage
fn update() -> Result<(), Error> {
    let token = std::env::var("DOWNLOAD_AUTH_TOKEN")
        .expect("DOWNLOAD_AUTH_TOKEN needs to the best in your environment");
    let status = self_update::backends::github::Update::configure()
        .repo_owner("PrecisionNutrition")
        .repo_name("pndev")
        .bin_name("pndev-linux-amd64")
        .auth_token(&token)
        .show_download_progress(true)
        .current_version(self_update::cargo_crate_version!())
        .build()?
        .update()?;

    println!("Update status: `{}`!", status.version());

    let exe_path = std::env::current_exe().expect("not executable");

    let metadata = std::fs::metadata(&exe_path)?;
    let mut perms = metadata.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&exe_path, perms)?;

    Ok(())
}
