use ansi_term::Colour::Green;
use ansi_term::Colour::Red;
use log::trace;
use std::process::Command;

use dns_lookup::lookup_host;

use failure::{bail, Error};

const APPS: &[&str] = &["git", "nix", "docker", "docker-compose"];
const HOSTNAME: &str = "es-dev.precisionnutrition.com";

/// Runs all checks
pub fn all() -> Result<(), Error> {
    trace!("pn_doctor called");
    for &app in APPS {
        if !check_app_installed(app) {
            bail!("{} not installed, run pndev doctor for help", app);
        }
    }

    if !check_host() {
        bail!("es-dev not configured, run pndev doctor for help");
    }

    Ok(())
}

pub fn doctor() -> Result<(), Error> {
    trace!("check_all called");

    for &app in APPS {
        if check_app_installed(app) {
            println!("{} {} installed", Green.paint("✓"), app);
        } else {
            println!("{} {} not installed", Red.paint("✗"), app);
        }
    }

    if check_host() {
        println!(
            "{} es-dev.precisionnutrition.com resolves",
            Green.paint("✓")
        );
    } else {
        println!(
            "{} es-dev.precisionnutrition.com does not resolve",
            Red.paint("✗")
        );
    }

    if check_github() {
        println!("{} github ssh access allowed", Green.paint("✓"));
    } else {
        println!("{} github ssh access not allowed", Red.paint("✗"));
    }

    if check_anonymize_creds() {
        println!("{} ~/.pn_anonymize_creds present", Green.paint("✓"));
    } else {
        println!("{} ~/.pn_anonymize_creds missing", Red.paint("✗"));
    }

    Ok(())
}

fn check_anonymize_creds() -> bool {
    let mut path = dirs::home_dir().unwrap();
    path.push(".pn_anonymize_creds");

    path.exists()
}

fn check_app_installed(command: &str) -> bool {
    let result = Command::new(command).args(["--version"]).output();

    result.is_ok()
}

fn check_host() -> bool {
    lookup_host(HOSTNAME).is_ok()
}

fn check_github() -> bool {
    let result = Command::new("ssh").args(["-T", "git@github.com"]).output();

    match result {
        // ssh -T returns 1 even if auth works
        Ok(output) => output.status.code().unwrap() % 255 == 1,
        Err(_) => false,
    }
}
