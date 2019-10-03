use failure::Error;
use std::path::Path;

use log::info;
use log::trace;

use failure::bail;

use dirs::home_dir;

use crate::check;
use crate::git;
use crate::shell;

const APPS: [&str; 5] = [
    "eternal-sledgehammer",
    "es-student",
    "fitpro",
    "es-certification",
    "payment-next",
];

#[derive(Debug)]
pub struct Command {
    name: Option<String>,
    all: bool,
    docker_only: bool,
}

impl Command {
    pub const fn new() -> Self {
        Command {
            name: None,
            all: false,
            docker_only: false,
        }
    }

    pub fn shell() -> Result<(), Error> {
        Self::new()
            .check()
            .and_then(Command::up)
            .and_then(Command::_nix)?;

        Ok(())
    }

    pub fn start(docker_only: bool) -> Result<(), Error> {
        trace!("start command");

        Command::new()
            .docker_only(docker_only)
            .check()
            .and_then(Command::up)
            .and_then(Command::_start)?;

        trace!("start command done");

        Ok(())
    }

    pub fn stop() -> Result<(), Error> {
        trace!("stop command");

        Command::new().check().and_then(Command::_stop)?;

        trace!("stop command done");

        Ok(())
    }

    pub fn ps() -> Result<(), Error> {
        trace!("ps command");

        Command::new().check().and_then(Command::_ps)?;

        trace!("ps command done");

        Ok(())
    }

    pub fn prepare() -> Result<(), Error> {
        trace!("anonymize command");

        Command::new()
            .check()
            .and_then(Command::up)
            .and_then(Command::_has_creds)
            .and_then(Command::_prepare)?;

        Ok(())
    }

    pub fn clone(name: Option<String>, all: bool) -> Result<(), Error> {
        trace!("clone command");

        Command::new()
            .name(name)
            .all(all)
            .check()
            .and_then(Command::up)
            .and_then(Command::_clone)?;

        info!("Clone completed");

        Ok(())
    }

    pub fn all(&mut self, all: bool) -> &mut Self {
        self.all = all;
        self
    }

    pub fn docker_only(&mut self, docker_only: bool) -> &mut Self {
        self.docker_only = docker_only;
        self
    }

    pub fn name(&mut self, name: Option<String>) -> &mut Self {
        self.name = name;
        self
    }

    pub fn check(&self) -> Result<&Self, Error> {
        check::all()?;

        Ok(self)
    }

    pub fn up(&self) -> Result<&Self, Error> {
        shell::docker_up()?;

        Ok(self)
    }

    pub fn _nix(&self) -> Result<&Self, Error> {
        trace!("shell started ");
        shell::nix()?;

        trace!("shell closed");
        Ok(self)
    }

    fn _start(&self) -> Result<&Self, Error> {
        if self.docker_only {
            info!("Starting only docker services");
        } else if Path::new("Gemfile.lock").exists() {
            shell::forego_start()?;
        } else if Path::new("ember-cli-build.js").exists() {
            shell::ember_start()?;
        } else {
            bail!("No Ruby or Ember app found")
        }

        Ok(self)
    }

    fn _stop(&self) -> Result<&Self, Error> {
        shell::docker_down()?;
        Ok(self)
    }

    fn _ps(&self) -> Result<&Self, Error> {
        println!("Docker ps output:");

        shell::docker_ps()?;

        Ok(self)
    }

    fn _has_creds(&self) -> Result<&Self, Error> {
        let mut path = home_dir().unwrap();
        path.push(".pn_anonymize_creds");

        if !path.exists() {
            bail!("Please create ~/.pn_anonymize_creds")
        }

        Ok(self)
    }

    fn _prepare(&self) -> Result<&Self, Error> {
        if Path::new("Gemfile.lock").exists() {
            shell::npm_rebuild_deps()?;

            shell::rails_migrate()?;

            shell::rails_anonymize()?;

            shell::rails_bootstrap()?;
        } else {
            bail!("No Gemfile found, are you in the right directory?")
        }

        Ok(self)
    }

    fn _clone(&self) -> Result<&Self, Error> {
        if self.all {
            for &app in &APPS {
                println!("Cloning {}", app);
                git::clone(app)?;
            }
        } else {
            match &self.name {
                Some(name) => {
                    println!("Cloning {}", name);
                    git::clone(&name[..])?;
                }
                None => bail!("Please specify an app name or --all"),
            }
        };

        Ok(self)
    }
}
