use crate::config;
use ansi_term::Colour::{Green, Red, Yellow};
use failure::{bail, Error};
use log::trace;
use std::fs;
use std::process::Command;

/// Clones a github repo from the PN org
pub fn clone(name: &str) -> Result<(), Error> {
    let dest = config::Config::new().repo_path();
    let app_path = format!("git@github.com:PrecisionNutrition/{}.git", name);
    let dest_path = format!("{}/{}", dest, name);
    let args = ["clone", "--recurse-submodules", &app_path, &dest_path];

    fs::create_dir_all(dest)?;

    let result = Command::new("git").args(&args).output();

    trace!("running git {:?}", result);

    match result {
        Ok(output) => {
            if output.status.code().unwrap() % 255 == 0 {
                Ok(())
            } else {
                bail!("{}", std::str::from_utf8(&output.stderr).unwrap())
            }
        }
        Err(err) => bail!("{} error", err),
    }
}

/// Updates a github repo from the PN org
pub fn update(name: &str) -> Result<(), Error> {
    let dest = config::Config::new().repo_path();
    let dest_path = format!("{}/{}", dest, name);
    let args = ["pull", "--ff"];

    std::env::set_current_dir(dest_path)?;

    let result = Command::new("git").args(&args).output();

    trace!("running git {:?}", result);

    match result {
        Ok(output) => {
            if output.status.code().unwrap() % 255 == 0 {
                Ok(())
            } else {
                bail!("{}", std::str::from_utf8(&output.stderr).unwrap())
            }
        }
        Err(err) => bail!("{} error", err),
    }
}

/// Checks out a pr
pub fn review(name: &str, pr: &str) -> Result<(), Error> {
    let dest = config::Config::new().repo_path();
    let dest_path = format!("{}/{}", dest, name);
    let args = ["fetch"];

    std::env::set_current_dir(dest_path)?;

    run_git_command(&args)?;

    let args2 = ["ls-remote", "--exit-code", "origin", pr];
    match run_git_command(&args2) {
        Ok(_output) => {
            pr_checkout(name, pr);
        }

        Err(_err) => {
            println!(
                "{} remote branch not found for {}:{}",
                Yellow.paint("⚠"),
                name,
                pr
            );
        }
    };

    Ok(())
}

fn pr_checkout(name: &str, pr: &str) {
    let origin = format!("origin/{}", pr);
    let args = ["checkout", "-b", pr, &origin];
    match run_git_command(&args) {
        Ok(_output) => {
            println!(
                "{} successfully checked out {}:{}",
                Green.paint("✓"),
                name,
                pr
            );
        }
        Err(err) => {
            println!(
                "{} Error during checkout of {}:{}\n{}",
                Red.paint("✗"),
                name,
                pr,
                err
            );
        }
    };
}

fn run_git_command(args: &[&str]) -> Result<(), Error> {
    let result = Command::new("git").args(args).output();

    trace!("running git {:?}", result);

    match result {
        Ok(output) => {
            if output.status.code().unwrap() % 255 == 0 {
                Ok(())
            } else {
                bail!("{}", std::str::from_utf8(&output.stderr).unwrap())
            }
        }
        Err(err) => bail!("{} error", err),
    }
}
