use std::process::{Command, ExitStatus};
use failure::Error;
use failure::bail;
use std::path::Path;
use crate::git;
use log::{info};

pub fn nix_shell() -> std::io::Result<ExitStatus> {
  let exe = "nix-shell";

  Command::new(exe).spawn()?.wait()
}

pub fn pndev_setup() -> Result<(), Error> {
  let path = format!("{}/pndev", git::pn_repos_path());

  if Path::new(&path[..]).exists() {
    info!("pndev already cloned, if you want to update run git update in /DEV/PN/pndev");
    Ok(())
  } else {
    git::clone("pndev")
  }
}

pub fn docker_up() -> Result<ExitStatus, Error> {
  pndev_setup()?;
  let mut args = vec!["-f"];

  let pndev_path = format!("{}/pndev/catalog/docker-compose.yml", git::pn_repos_path());
  args.push(&pndev_path);

  args.extend_from_slice(&["up", "--no-recreate", "-d"]);

  let status = Command::new("docker-compose").args(&args).spawn()?.wait()?;

  if !(status.code().unwrap() % 255 == 0) {
    bail!("docker up failed");
  } else {
    Ok(status)
  }
}

pub fn docker_down() -> Result<ExitStatus, Error> {
  pndev_setup()?;

  let mut args = vec!["-f"];

  let pndev_path = format!("{}/pndev/catalog/docker-compose.yml", git::pn_repos_path());
  args.push(&pndev_path);

  args.extend_from_slice(&["down"]);

  let status = Command::new("docker-compose").args(&args).spawn()?.wait()?;

  if !(status.code().unwrap() % 255 == 0) {
    bail!("docker down failed");
  } else {
    Ok(status)
  }
}

pub fn docker_ps() -> Result<ExitStatus, Error> {
  pndev_setup()?;

  let mut args = vec!["-f"];

  let pndev_path = format!("{}/pndev/catalog/docker-compose.yml", git::pn_repos_path());
  args.push(&pndev_path);

  args.extend_from_slice(&["ps"]);

  let status = Command::new("docker-compose").args(&args).spawn()?.wait()?;

  if !(status.code().unwrap() % 255 == 0) {
    bail!("docker ps failed");
  } else {
    Ok(status)
  }
}

pub fn forego_start() -> Result<ExitStatus, Error> {
  let args = ["--run", "bundle && yarn && bundle exec rails db:create db:migrate && pnforego start"];

  let status = Command::new("nix-shell").args(&args).spawn()?.wait()?;
  if !(status.code().unwrap() % 255 == 0) {
    bail!("forego start failed")
  } else {
    Ok(status)
  }
}

pub fn rails_migrate() -> Result<ExitStatus, Error> {
  let args = ["--run", "bundle && yarn && bundle exec rails db:create db:migrate"];

  let status = Command::new("nix-shell").args(&args).spawn()?.wait()?;

  if !(status.code().unwrap() % 255 == 0) {
    bail!("migrate failed")
  } else {
    Ok(status)
  }
}

pub fn rails_bootstrap() -> Result<ExitStatus, Error> {
  let args = ["--run", "bundle && yarn && RAILS_ENV=development bundle exec cucumber bootstrap"];

  let status = Command::new("nix-shell").args(&args).spawn()?.wait()?;

  if !(status.code().unwrap() % 255 == 0) {
    bail!("bootstrap failed")
  } else {
    Ok(status)
  }
}

pub fn rails_anonymize() -> Result<ExitStatus, Error> {
  let args = ["--run", "bundle && yarn && bundle exec rails db:anonymize"];

  let status = Command::new("nix-shell").args(&args).spawn()?.wait()?;

  if !(status.code().unwrap() % 255 == 0) {
    bail!("anonymize failed")
  } else {
    Ok(status)
  }
}

pub fn ember_start() -> Result<ExitStatus, Error> {
  let args = ["--run", "yarn && yarn exec ember server"];

  let status = Command::new("nix-shell").args(&args).spawn()?.wait()?;

  if !(status.code().unwrap() % 255 == 0) {
    bail!("ember s failed")
  } else {
    Ok(status)
  }
}

pub fn npm_rebuild_deps() -> Result<ExitStatus, Error> {
  let args = ["--run", "npm rebuild xxhash node-sass"];

  let status = Command::new("nix-shell").args(&args).spawn()?.wait()?;
  if !(status.code().unwrap() % 255 == 0) {
    bail!("rebuilding node deps")
  } else {
    Ok(status)
  }
}
