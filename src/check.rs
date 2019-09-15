use log::trace;
use std::process::Command;
use ansi_term::Colour::Green;
use ansi_term::Colour::Red;

pub fn check_all() {
  trace!("check::check_all called");

  if check_installed("git") {
    println!("{} git installed", Green.paint("✓"));
  } else {
    println!("{} git not installed", Red.paint("✗"));
  }

  if check_installed("nix") {
    println!("{} nix installed", Green.paint("✓"));
  } else {
    println!("{} nix not installed", Red.paint("✗"));
  }

  if check_installed("docker") {
    println!("{} docker installed", Green.paint("✓"));
  } else {
    println!("{} docker not installed", Red.paint("✗"));
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
