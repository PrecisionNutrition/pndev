use log::trace;
use failure::{Error, bail};
use std::process::{Command};
use std::fs;
use dirs::home_dir;

/// Clones a github repo from the PN org
pub fn clone(name: &str) -> Result<(), Error> {
  let dest = pn_repos_path();
  let app_path = format!("git@github.com:PrecisionNutrition/{}.git", name);
  let dest_path = format!("{}/{}", dest, name);
  let args = ["clone", &app_path, &dest_path];

  fs::create_dir_all(dest)?;

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

/// Calculates path for local repo clones
pub fn pn_repos_path() -> String {
  let mut home_path =  home_dir().unwrap();
  home_path.push("DEV/PN");
  home_path.into_os_string().into_string().unwrap()
}
