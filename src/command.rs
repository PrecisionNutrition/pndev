use failure::Error;
use std::path::Path;

use log::info;
use log::trace;

use failure::bail;

use dirs::home_dir;

use crate::check;
use crate::git;
use crate::shell;

const APPS: &[&str] = &[
    "eternal-sledgehammer",
    "es-student",
    "fitpro",
    "es-certification",
    "payment-next",
    "courier",
    "owners-manual",
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
        Self::new().check()?._up()?._nix()?;

        Ok(())
    }

    pub fn start(docker_only: bool) -> Result<(), Error> {
        trace!("start command");

        Command::new()
            .docker_only(docker_only)
            .check()?
            ._up()?
            ._start()?;

        trace!("start command done");

        Ok(())
    }

    pub fn up() -> Result<(), Error> {
        trace!("up command");

        Command::new()
            .check()?
            ._up()?;

        trace!("up command done");

        Ok(())
    }

    pub fn down() -> Result<(), Error> {
        trace!("down command");

        Command::new().check()?._down()?;

        trace!("down command done");

        Ok(())
    }

    pub fn ps() -> Result<(), Error> {
        trace!("ps command");

        Command::new().check()?._ps()?;

        trace!("ps command done");

        Ok(())
    }

    pub fn rebuild() -> Result<(), Error> {
        trace!("rebuild command");

        Command::new().check()?._rebuild()?;

        trace!("rebuild command done");

        Ok(())
    }

    pub fn reset() -> Result<(), Error> {
        trace!("reset command");

        Command::new().check()?._reset()?;

        trace!("reset command done");

        Ok(())
    }

    pub fn prepare(quick: bool) -> Result<(), Error> {
        trace!("anonymize command");

        Command::new()
            .check()?
            ._up()?
            ._has_creds()?
            ._prepare(quick)?;

        Ok(())
    }

    pub fn clone(name: Option<String>, all: bool) -> Result<(), Error> {
        trace!("clone command");

        Command::new().name(name).all(all).check()?._up()?._clone()?;

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

    pub fn _up(&self) -> Result<&Self, Error> {
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

    fn _down(&self) -> Result<&Self, Error> {
        shell::docker_down()?;
        Ok(self)
    }

    fn _ps(&self) -> Result<&Self, Error> {
        println!("Docker ps output:");

        shell::docker_ps()?;

        Ok(self)
    }

    fn _rebuild(&self) -> Result<&Self, Error> {
        // pull new docker configs
        git::update("pndev")?;

        // stop docker
        shell::docker_down()?;

        // rebuild container
        shell::docker_rebuild()?;

        // ensure new containers are used
        shell::docker_up_recreate()?;

        Ok(self)
    }

    fn _reset(&self) -> Result<&Self, Error> {
        trace!("calling _reset and passing to shell");
        shell::reset()?;

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

    fn _prepare(&self, quick: bool) -> Result<&Self, Error> {
        if Path::new("Gemfile.lock").exists() {
            shell::npm_rebuild_deps()?;


            shell::rails_set_env()?;
            shell::rails_db_drop()?;
            shell::rails_migrate()?;

            if !quick {
                shell::rails_anonymize()?;
            }

            shell::rails_bootstrap()?;
        } else {
            bail!("No Gemfile found, are you in the right directory?")
        }

        Ok(self)
    }

    fn _clone(&self) -> Result<&Self, Error> {
        if self.all {
            for &app in APPS {
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
