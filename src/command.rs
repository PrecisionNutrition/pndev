use failure::Error;
use std::path::Path;

use log::info;
use log::trace;

use failure::bail;

use dirs::home_dir;

use ansi_term::Colour::Red;
use dialoguer::Confirm;

use crate::check;
use crate::git;
use crate::parse;
use crate::shell;
use crate::ResetType;

const REPOS: &[&str] = &[
    "eternal-sledgehammer",
    "es-student",
    "es-admin",
    "fitpro",
    "es-certification",
    "payment-next",
    "courier",
    "owners-manual",
    "profile-engine",
    "academy",
];

const APPS: &[&str] = &[
    "eternal-sledgehammer",
    "es-student",
    "fitpro",
    "es-certification",
    "es-admin",
    "payment-next",
    "academy",
];

#[derive(Debug)]
pub struct Command {
    name: Option<String>,
    arguments: Vec<String>,
    pr: Option<String>,
    all: bool,
    docker_only: bool,
}

impl Command {
    pub const fn new() -> Self {
        Self {
            name: None,
            pr: None,
            all: false,
            docker_only: false,
            arguments: vec![],
        }
    }

    pub fn shell(arguments: Vec<String>) -> Result<(), Error> {
        trace!("shell command: {:?}", arguments);

        Self::new().arguments(arguments).check()?._up()?._nix()?;

        Ok(())
    }

    pub fn start(docker_only: bool) -> Result<(), Error> {
        trace!("start command");

        Self::new()
            .docker_only(docker_only)
            .check()?
            ._up()?
            ._start()?;

        trace!("start command done");

        Ok(())
    }

    pub fn up() -> Result<(), Error> {
        trace!("up command");

        Self::new().check()?._up()?;

        trace!("up command done");

        Ok(())
    }

    pub fn down() -> Result<(), Error> {
        trace!("down command");

        Self::new().check()?._down()?;

        trace!("down command done");

        Ok(())
    }

    pub fn ps() -> Result<(), Error> {
        trace!("ps command");

        Self::new().check()?._ps()?;

        trace!("ps command done");

        Ok(())
    }

    pub fn reset(docker_or_local: ResetType) -> Result<(), Error> {
        trace!("reset command");

        match docker_or_local {
            ResetType::Scratch => {
                if Confirm::new()
                    .with_prompt(Red.paint("DANGER: pndev reset scratch will reset all your local changes. Use with care").to_string())
                    .default(false)
                    .interact()?
                {
                    Self::new().check()?._scratch()?._rebuild()?._reset()?
                } else {
                    bail!("User abort");
                }
            }
            ResetType::Docker => Self::new().check()?._rebuild()?,
            ResetType::Deps => Self::new().check()?._reset()?,
        };

        trace!("reset command done");

        Ok(())
    }

    pub fn prepare(big: bool) -> Result<(), Error> {
        trace!("anonymize command");
        let args = if big { vec!["--big".into()] } else { vec![] };

        Self::new()
            .arguments(args)
            .check()?
            ._up()?
            ._has_creds()?
            ._prepare()?;

        Ok(())
    }

    pub fn clone(name: Option<String>, all: bool) -> Result<(), Error> {
        trace!("clone command");

        Self::new().name(name).all(all).check()?._up()?._clone()?;

        info!("Clone completed");

        Ok(())
    }

    pub fn review(pr: Option<String>, name: Option<String>) -> Result<(), Error> {
        trace!("review command");

        Self::new().name(name).pr(pr).check()?._up()?._review()?;

        info!("Review completed");

        Ok(())
    }

    pub fn gh() -> Result<(), Error> {
        trace!("gh command");

        git::open()?;

        Ok(())
    }

    pub fn run(name: Option<String>, arguments: Vec<String>) -> Result<(), Error> {
        trace!("run command");

        Self::new()
            .name(name)
            .arguments(arguments)
            .check()?
            ._up()?
            ._run()?;

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

    pub fn pr(&mut self, pr: Option<String>) -> &mut Self {
        self.pr = pr;
        self
    }

    pub fn arguments(&mut self, arguments: Vec<String>) -> &mut Self {
        self.arguments = arguments;
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

    fn _args_to_string(&self) -> String {
        self.arguments
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .join(" ")
    }

    pub fn _nix(&self) -> Result<&Self, Error> {
        trace!("shell started");

        shell::nix(&self._args_to_string())?;

        trace!("shell closed");

        Ok(self)
    }

    fn _start(&self) -> Result<&Self, Error> {
        if self.docker_only {
            info!("Starting only docker services");
        } else if Path::new(".pndev/start").exists() {
            self._run_command("start")?;
        } else if Path::new("pndev.toml").exists() {
            self._run_pndev_toml_command("start")?;
        } else if Path::new("ember-cli-build.js").exists() {
            shell::ember_start()?;
        } else {
            bail!("No Ruby or Ember app found")
        }

        Ok(self)
    }

    fn _run(&self) -> Result<&Self, Error> {
        trace!("calling _run and passing to shell");

        match &self.name {
            Some(name) => {
                if Path::new(&["./.pndev", name].join("/")).exists() {
                    self._run_command(name)?;
                } else if Path::new("pndev.toml").exists() {
                    self._run_pndev_toml_command(name)?;
                } else {
                    bail!("Command {} not found", name);
                }
            }
            None => bail!("Please specify a command to run"),
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

    fn _scratch(&self) -> Result<&Self, Error> {
        if Path::new(".pndev/scratch").exists() {
            self._run_command("scratch")?;
        } else {
            self._run_pndev_toml_command("scratch")?;
        }

        Ok(self)
    }

    fn _prepare(&self) -> Result<&Self, Error> {
        self._run_command("prepare")?;

        Ok(self)
    }

    fn _clone(&self) -> Result<&Self, Error> {
        if self.all {
            for &app in REPOS {
                println!("Cloning {}", app);
                git::clone(app)?;
            }
        } else {
            match &self.name {
                Some(name) => {
                    println!("Cloning {}", name);
                    git::clone(name)?;
                }
                None => bail!("Please specify an app name or --all"),
            }
        }

        Ok(self)
    }

    fn _review(&self) -> Result<&Self, Error> {
        match &self.pr {
            Some(pr) => match &self.name {
                Some(name) => {
                    info!("Pulling {}:{} for review", name, pr);
                    git::review(name, pr)?;
                }
                None => {
                    for &app in APPS {
                        info!("Pulling {}:{} for review", app, pr);
                        git::review(app, pr)?;
                    }
                }
            },
            None => bail!("Please specify a Pull Request (branch name)"),
        }

        Ok(self)
    }

    fn _run_pndev_toml_command(&self, name: &str) -> Result<(), Error> {
        if Path::new("pndev.toml").exists() {
            let command = parse::config()?;
            match command.get(name) {
                Some(cmd) => match cmd.as_str() {
                    Some(cmd) => {
                        info!("executing {} command: {}", name, cmd);
                        let args = self._args_to_string();

                        shell::nix(&[cmd, &args].join(" "))?;
                    }
                    None => bail!("Invalid {} command", name),
                },
                None => bail!("Command {} not found", name),
            }
        } else {
            bail!("pndev.toml not found")
        }

        Ok(())
    }

    fn _run_command(&self, command: &str) -> Result<(), Error> {
        let args = self._args_to_string();
        let cmd = ["./.pndev", command].join("/");

        info!("executing command {} with args {:?} ", cmd, args);

        shell::nix(&[cmd, args].join(" "))?;
        Ok(())
    }
}
