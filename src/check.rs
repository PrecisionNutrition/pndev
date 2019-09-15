use log::trace;
use std::process::Command;
use ansi_term::Colour::Green;
use ansi_term::Colour::Red;

use dns_lookup::lookup_host;

use failure::{bail, Error};

const APPS: [&str; 3] = ["git", "nix", "docker"];
const HOSTNAME: &str = "es-dev.precisionnutrition.com";

pub fn pn_doctor() -> Result<(), Error> {
  trace!("pn_doctor called");
  for app in APPS.iter() {
    if !check_app_installed(app) {
      bail!("{} not installed, run pndev check for help", app);
    }
  }

  if !check_host() {
    bail!("es-dev not configured, run pndev check for help");
  }

  Ok(())
}

pub fn check_all() -> Result<(), Error> {
  trace!("check_all called");

  for app in APPS.iter() {
    if check_app_installed(app) {
      println!("{} {} installed", Green.paint("✓"), app);
    } else {
      println!("{} {} not installed", Red.paint("✗"), app);
    }
  }

  if check_host() {
    println!("{} es-dev.precisionnutrition.com resolves", Green.paint("✓"));
  } else {
    println!("{} es-dev.precisionnutrition.com does not resolve", Red.paint("✗"));
  }

  if check_github() {
    println!("{} github ssh access allowed", Green.paint("✓"));
  } else {
    println!("{} githus ssh access not allowed", Red.paint("✗"));
  }

  Ok(())
}

fn check_app_installed(command: &str) -> bool {
  let result = Command::new(command)
    .args(&["--version"])
    .output();
  
  match result {
    Ok(_) => true,
    Err(_) => false,
  }
}

fn check_host() -> bool {
  match lookup_host(HOSTNAME) {
    Ok(_) => true,
    Err(_) => false,
  }
}

fn check_github() -> bool {
  let result = Command::new("ssh")
    .args(&["-T", "git@github.com"])
    .output();

  match result {
    // ssh -T returns 1 even if auth works
    Ok(output) => output.status.code().unwrap() % 255 == 1,
    Err(_) => false,
  }
}
