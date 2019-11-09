use crate::config;
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
