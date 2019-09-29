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
            bail!("docker up failed");
        }
    }

    pub fn check_setup() -> Result<(), Error> {
        let path = format!("{}/pndev", git::pn_repos_path());

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
        }
    }
}

pub fn nix() -> Result<ExitStatus, Error> {
    Shell::new().cmd("nix-shell").spawn()
}

pub fn docker_up() -> Result<ExitStatus, Error> {
    let mut args = vec!["-f"];

    let pndev_path = format!("{}/pndev/catalog/docker-compose.yml", git::pn_repos_path());
    args.push(&pndev_path);

    args.extend_from_slice(&["up", "--no-recreate", "-d"]);

    Shell::new().cmd("docker-compose").args(args).spawn()
}

pub fn docker_down() -> Result<ExitStatus, Error> {
    let mut args = vec!["-f"];

    let pndev_path = format!("{}/pndev/catalog/docker-compose.yml", git::pn_repos_path());
    args.push(&pndev_path);

    args.extend_from_slice(&["down"]);

    Shell::new().cmd("docker-compose").args(args).spawn()
}

pub fn docker_ps() -> Result<ExitStatus, Error> {
    let mut args = vec!["-f"];

    let pndev_path = format!("{}/pndev/catalog/docker-compose.yml", git::pn_repos_path());
    args.push(&pndev_path);

    args.extend_from_slice(&["ps"]);

    Shell::new().cmd("docker-compose").args(args).spawn()
}

pub fn forego_start() -> Result<ExitStatus, Error> {
    let args = vec![
        "--run",
        "bundle && yarn && bundle exec rails db:create db:migrate && pnforego start",
    ];

    Shell::new().cmd("nix-shell").args(args).spawn()
}

pub fn rails_migrate() -> Result<ExitStatus, Error> {
    let args = vec![
        "--run",
        "bundle && yarn && bundle exec rails db:create db:migrate",
    ];

    Shell::new().cmd("nix-shell").args(args).spawn()
}

pub fn rails_bootstrap() -> Result<ExitStatus, Error> {
    let args = vec![
        "--run",
        "bundle && yarn && RAILS_ENV=development bundle exec cucumber bootstrap",
    ];

    Shell::new().cmd("nix-shell").args(args).spawn()
}

pub fn rails_anonymize() -> Result<ExitStatus, Error> {
    let args = vec!["--run", "bundle && yarn && bundle exec rails db:anonymize"];

    Shell::new().cmd("nix-shell").args(args).spawn()
}

pub fn ember_start() -> Result<ExitStatus, Error> {
    let args = vec!["--run", "yarn && yarn exec ember server"];

    Shell::new().cmd("nix-shell").args(args).spawn()
}

pub fn npm_rebuild_deps() -> Result<ExitStatus, Error> {
    let args = vec!["--run", "npm rebuild xxhash node-sass"];

    Shell::new().cmd("nix-shell").args(args).spawn()
}
