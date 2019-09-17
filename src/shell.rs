use std::process::{Command, ExitStatus};
use std::io::Result;
use sys_info::os_type;

pub fn nix_shell() -> Result<ExitStatus> {
  let exe = "nix-shell";

  Command::new(exe).spawn()?.wait()
}

pub fn docker_up() -> Result<ExitStatus> {
  let mut args = vec!["-f"];

  if os_type().unwrap() == "Darwin" {
    args.push("docker-compose-minimal-osx.yml");
  } else {
    args.push("docker-compose-minimal.yml");
  }

  args.extend_from_slice(&["up", "--no-recreate", "-d"]);

  Command::new("docker-compose").args(&args).spawn()?.wait()
}

pub fn docker_down() -> Result<ExitStatus> {
  let mut args = vec!["-f"];

  if os_type().unwrap() == "Darwin" {
    args.push("docker-compose-minimal-osx.yml");
  } else {
    args.push("docker-compose-minimal.yml");
  }

  args.extend_from_slice(&["down"]);

  Command::new("docker-compose").args(&args).spawn()?.wait()
}

pub fn forego_start() -> Result<ExitStatus> {
  let args = ["--run", "bundle && yarn && pnforego start"];

  Command::new("nix-shell").args(&args).spawn()?.wait()
}

pub fn rails_migrate() -> Result<ExitStatus> {
  let args = ["--run", "bundle && yarn && bundle exec rails db:create db:migrate"];

  Command::new("nix-shell").args(&args).spawn()?.wait()
}

pub fn rails_bootstrap() -> Result<ExitStatus> {
  let args = ["--run", "bundle && yarn && RAILS_ENV=development bundle exec cucumber bootstrap"];

  Command::new("nix-shell").args(&args).spawn()?.wait()
}

pub fn rails_anonymize() -> Result<ExitStatus> {
  let args = ["--run", "bundle && yarn && bundle exec rails db:anonymize"];

  Command::new("nix-shell").args(&args).spawn()?.wait()
}

pub fn ember_start() -> Result<ExitStatus> {
  let args = ["--run", "yarn && yarn exec ember server"];

  Command::new("nix-shell").args(&args).spawn()?.wait()
}
