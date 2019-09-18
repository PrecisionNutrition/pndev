use log::trace;
use failure::{Error, bail};
use std::process::{Command};

pub fn clone(name: &str) -> Result<(), Error> {
  let app_path = format!("git@github.com:PrecisionNutrition/{}.git", name);
  let args = ["clone", &app_path];

  let result = Command::new("git")
    .args(&args)
    .output();

  trace!("running git {:?}", result);

  match result {
    Ok(output) => {
      if output.status.code().unwrap() % 255 == 0 {
        Ok(())
      } else {
        bail!("{}", std::str::from_utf8(&output.stderr).unwrap())
      }
    },
    Err(err) => bail!("{} error", err),
  }
}
