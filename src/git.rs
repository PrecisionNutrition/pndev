use crate::config;
use ansi_term::Colour::{Green, Red, Yellow};
use failure::{bail, Error};
use lazy_static::lazy_static;
use log::trace;
use regex::Regex;
use std::fs;
use std::process::Command;

/// Clones a github repo from the PN org
pub fn clone(name: &str) -> Result<(), Error> {
    let dest = config::Config::new().repo_path();
    let app_path = format!("git@github.com:PrecisionNutrition/{name}.git");
    let dest_path = format!("{dest}/{name}");
    let args = ["clone", "--recurse-submodules", &app_path, &dest_path];

    fs::create_dir_all(dest)?;

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

/// Updates a github repo from the PN org
pub fn update(name: &str) -> Result<(), Error> {
    let dest = config::Config::new().repo_path();
    let dest_path = format!("{dest}/{name}");
    let args = ["pull", "--ff"];

    std::env::set_current_dir(dest_path)?;

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

/// Checks out a pr
pub fn review(name: &str, pr: &str) -> Result<(), Error> {
    let dest = config::Config::new().repo_path();
    let dest_path = format!("{dest}/{name}");
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
    let origin = format!("origin/{pr}");
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

fn extract_repo_name(input: &str) -> Option<[&str; 2]> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r".*:(?P<org_name>.*)/(?P<repo_name>.*)(\.git)?").unwrap();
    }

    trace!("running input {:?}", input);

    RE.captures(input).and_then(|cap| {
        trace!("running regex {:?}", cap);
        cap.name("repo_name").and_then(|repo_name| {
            trace!("running regex2 {:?}", repo_name);
            cap.name("org_name")
                .map(|org_name| [org_name.as_str(), repo_name.as_str()])
        })
    })
}

// Opens github repo in PN org
pub fn open() -> Result<(), Error> {
    let args = ["remote", "get-url", "origin"];
    let result = Command::new("git").args(args).output();

    trace!("running git {:?}", result);

    match result {
        Ok(output) => {
            if output.status.code().unwrap() % 255 == 0 {
                let remote = std::str::from_utf8(&output.stdout).unwrap();

                match extract_repo_name(remote) {
                    Some([org_name, repo_name]) => {
                        let repo_url = format!("https://github.com/{org_name}/{repo_name}");

                        trace!("repo_url {:?}", repo_url);

                        open::that(repo_url).unwrap();

                        Ok(())
                    }
                    None => bail!("Could not determine repo url"),
                }
            } else {
                bail!("{}", std::str::from_utf8(&output.stderr).unwrap())
            }
        }
        Err(err) => bail!("{} error", err),
    }
}
