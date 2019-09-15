use log::trace;
use std::process::Command;

pub fn check_all() {
  trace!("check::check_all called");

  if check_installed("git") {
    trace!("git installed")
  } else {
    trace!("git not installed")
  }

  if check_installed("nix") {
    trace!("nix installed")
  } else {
    trace!("nix not installed")
  }

  if check_installed("docker") {
    trace!("docker installed")
  } else {
    trace!("docker not installed")
  }
}

fn check_installed(command: &str) -> bool {
  let result = Command::new(command)
    .args(&["--version"])
    .output();
  
  match result {
    Ok(_) => true,
    Err(_) => false,
  }
}
