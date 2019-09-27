use std::process::{Command, ExitStatus};
use sys_info::os_type;
use failure::Error;
use failure::bail;

pub fn nix_shell() -> std::io::Result<ExitStatus> {
  let exe = "nix-shell";

  Command::new(exe).spawn()?.wait()
}

pub fn docker_up() -> Result<ExitStatus, Error> {
  let mut args = vec!["-f"];

  if os_type().unwrap() == "Darwin" {
    args.push("docker-compose-minimal-osx.yml");
  } else {
    args.push("docker-compose-minimal.yml");
  }

  args.extend_from_slice(&["up", "--no-recreate", "-d"]);

  let status = Command::new("docker-compose").args(&args).spawn()?.wait()?;

  if !(status.code().unwrap() % 255 == 0) {
    bail!("docker up failed");
  } else {
    Ok(status)
  }
}

pub fn docker_down() -> std::io::Result<ExitStatus> {
  let mut args = vec!["-f"];

  if os_type().unwrap() == "Darwin" {
    args.push("docker-compose-minimal-osx.yml");
  } else {
    args.push("docker-compose-minimal.yml");
  }

  args.extend_from_slice(&["down"]);

  Command::new("docker-compose").args(&args).spawn()?.wait()
}

pub fn forego_start() -> std::io::Result<ExitStatus> {
  let args = ["--run", "bundle && yarn && bundle exec rails db:create db:migrate && pnforego start"];

  Command::new("nix-shell").args(&args).spawn()?.wait()
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
