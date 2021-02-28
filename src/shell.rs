use crate::config;
use crate::git;
use failure::bail;
use failure::Error;
use log::info;
use log::trace;
use std::path::Path;
use std::process::{Command, ExitStatus};

#[derive(Debug)]
pub struct Shell<'a> {
    cmd: Option<String>,
    args: Vec<&'a str>,
    error_msg: &'a str,
}

impl<'a> Shell<'a> {
    pub fn new() -> Self {
        Shell::default()
    }

    pub fn cmd(&mut self, cmd: &str) -> &mut Self {
        self.cmd = Some(cmd.to_owned());
        self
    }

    pub fn args(&mut self, args: Vec<&'a str>) -> &mut Self {
        self.args = args;
        self
    }

    pub fn error_msg(&mut self, error_msg: &'a str) -> &mut Self {
        self.error_msg = error_msg;
        self
    }

    pub fn spawn(&mut self) -> Result<ExitStatus, Error> {
        Shell::check_setup()?;

        let cmd = match &self.cmd {
            Some(cmd) => cmd,
            None => bail!("missing cmd"),
        };

        trace!(
            "attempting to execute command {:?} with args {:?}",
            cmd,
            &self.args
        );

        let status = Command::new(cmd).args(&self.args).spawn()?.wait()?;

        trace!("command {:?} executed with args {:?}", cmd, &self.args);

        let code = status.code().unwrap();

        if code % 255 == 0 || code % 130 == 0 {
            Ok(status)
        } else {
            bail!(self.error_msg.to_owned());
        }
    }

    pub fn check_setup() -> Result<(), Error> {
        let path = format!("{}/pndev", config::Config::new().repo_path());

        if Path::new(&path).exists() {
            info!("pndev already cloned, if you want to update run git update in /DEV/PN/pndev");

            Ok(())
        } else {
            git::clone("pndev")
        }
    }
}

impl Default for Shell<'_> {
    fn default() -> Self {
        Self {
            cmd: None,
            args: vec![],
            error_msg: "Shell command failed",
        }
    }
}

pub fn nix() -> Result<ExitStatus, Error> {
    Shell::new().cmd("nix-shell").spawn()
}

pub fn docker_up() -> Result<ExitStatus, Error> {
    _docker_up(false)
}

pub fn docker_up_recreate() -> Result<ExitStatus, Error> {
    _docker_up(true)
}

fn _docker_up(force_recreate: bool) -> Result<ExitStatus, Error> {
    let mut args = vec!["-f"];

    let pndev_path = format!(
        "{}/pndev/catalog/docker-compose.yml",
        config::Config::new().repo_path()
    );

    args.push(&pndev_path);
    args.extend_from_slice(&["up", "-d"]);

    if force_recreate {
        args.push("--force-recreate");
    } else {
        args.push("--no-recreate");
    }

    Shell::new()
        .cmd("docker-compose")
        .args(args)
        .error_msg("Docker up failed")
        .spawn()
}

pub fn docker_down() -> Result<ExitStatus, Error> {
    let mut args = vec!["-f"];

    let pndev_path = format!(
        "{}/pndev/catalog/docker-compose.yml",
        config::Config::new().repo_path()
    );

    args.push(&pndev_path);
    args.extend_from_slice(&["down"]);

    Shell::new()
        .cmd("docker-compose")
        .args(args)
        .error_msg("Docker down failed")
        .spawn()
}

pub fn docker_ps() -> Result<ExitStatus, Error> {
    let mut args = vec!["-f"];

    let pndev_path = format!(
        "{}/pndev/catalog/docker-compose.yml",
        config::Config::new().repo_path()
    );

    args.push(&pndev_path);
    args.extend_from_slice(&["ps"]);

    Shell::new()
        .cmd("docker-compose")
        .args(args)
        .error_msg("Docker ps failed")
        .spawn()
}

pub fn docker_rebuild() -> Result<ExitStatus, Error> {
    let mut args = vec!["-f"];

    let pndev_path = format!(
        "{}/pndev/catalog/docker-compose.yml",
        config::Config::new().repo_path()
    );

    args.push(&pndev_path);
    args.extend_from_slice(&["build", "--no-cache"]);

    Shell::new()
        .cmd("docker-compose")
        .args(args)
        .error_msg("Docker rebuild failed")
        .spawn()
}

pub fn ember_start() -> Result<ExitStatus, Error> {
    let args = vec!["--run", "yarn && yarn exec ember server"];

    Shell::new().cmd("nix-shell").args(args).spawn()
}

pub fn reset() -> Result<ExitStatus, Error> {
    let args2 = vec![
        "-rf",
        ".nix-gems",
        "vendor/cache",
        "node_modules",
        ".nix-node",
    ];

    trace!("removing gems and node cache");

    Shell::new().cmd("rm").args(args2).spawn()
}

pub fn run(cmd: &str) -> Result<ExitStatus, Error> {
    let args = vec!["--run", cmd];

    Shell::new()
        .cmd("nix-shell")
        .args(args)
        .error_msg("Forego start failed")
        .spawn()
}
