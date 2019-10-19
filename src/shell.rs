use crate::config;
use crate::git;
use failure::bail;
use failure::Error;
use log::info;
use std::path::Path;
use std::process::{Command, ExitStatus};

#[derive(Debug)]
pub struct Shell<'a> {
    cmd: Option<String>,
    args: Vec<&'a str>,
    error_mgs: &'a str,
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

    pub fn error_mgs(&mut self, error_mgs: &'a str) -> &mut Self {
        self.error_mgs = error_mgs;
        self
    }

    pub fn spawn(&mut self) -> Result<ExitStatus, Error> {
        Shell::check_setup()?;

        let cmd = match &self.cmd {
            Some(cmd) => cmd,
            None => bail!("missing cmd"),
        };

        let status = Command::new(cmd).args(&self.args).spawn()?.wait()?;

        if status.code().unwrap() % 255 == 0 {
            Ok(status)
        } else {
            bail!(self.error_mgs.to_owned());
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
            error_mgs: "Shell command failed",
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
        .error_mgs("Docker up failed")
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
        .error_mgs("Docker down failed")
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
        .error_mgs("Docker ps failed")
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
        .error_mgs("Docker ps failed")
        .spawn()
}

pub fn forego_start() -> Result<ExitStatus, Error> {
    let args = vec![
        "--run",
        "bundle && yarn && bundle exec rails db:create db:migrate && pnforego start",
    ];

    Shell::new()
        .cmd("nix-shell")
        .args(args)
        .error_mgs("Forego start failed")
        .spawn()
}

pub fn rails_migrate() -> Result<ExitStatus, Error> {
    let args = vec![
        "--run",
        "bundle && yarn && bundle exec rails db:create db:migrate",
    ];

    Shell::new()
        .cmd("nix-shell")
        .args(args)
        .error_mgs("Rails migrate failed")
        .spawn()
}

pub fn rails_bootstrap() -> Result<ExitStatus, Error> {
    let args = vec![
        "--run",
        "bundle && yarn && RAILS_ENV=development bundle exec cucumber bootstrap",
    ];

    Shell::new()
        .cmd("nix-shell")
        .args(args)
        .error_mgs("Bootstrap failed")
        .spawn()
}

pub fn rails_anonymize() -> Result<ExitStatus, Error> {
    let args = vec!["--run", "bundle && yarn && bundle exec rails db:anonymize"];

    Shell::new()
        .cmd("nix-shell")
        .args(args)
        .error_mgs("Anonyimize failed")
        .spawn()
}

pub fn ember_start() -> Result<ExitStatus, Error> {
    let args = vec!["--run", "yarn && yarn exec ember server"];

    Shell::new().cmd("nix-shell").args(args).spawn()
}

pub fn npm_rebuild_deps() -> Result<ExitStatus, Error> {
    let args = vec!["--run", "npm rebuild xxhash node-sass"];

    Shell::new().cmd("nix-shell").args(args).spawn()
}
