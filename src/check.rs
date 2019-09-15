use log::trace;
use std::process::Command;
use ansi_term::Colour::Green;
use ansi_term::Colour::Red;

use dns_lookup::lookup_host;

const APPS: [&str; 3] = ["git", "nix", "docker"];
const HOSTNAME: &str = "es-dev.precisionnutrition.com";

pub fn pn_doctor() -> Result<(), &'static str> {
  for app in APPS.iter() {
    if !check_app_installed(app) {
      // seems bad
      // https://stackoverflow.com/questions/23975391/how-to-convert-a-string-into-a-static-str
      let err = format!("{} not installed", app);
      return Err(Box::leak(err.into_boxed_str()));
    }
  }

  Ok(())
}

pub fn check_all() {
  trace!("check::check_all called");

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

pub fn check_host() -> bool {
  match lookup_host(HOSTNAME) {
    Ok(_) => true,
    Err(_) => false,
  }
}
