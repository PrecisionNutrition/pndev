use log::trace;
use failure::{Error, bail};
use std::process::{Command};
use std::fs;
use dirs::home_dir;


pub fn clone(name: &str) -> Result<(), Error> {
  let mut home_path =  home_dir().unwrap();
  home_path.push("DEV/PN");

  println!("Cloning {} into {:#?}", name, home_path);

  let dest_root: String = format!("{}", home_path.into_os_string().into_string().unwrap());

  let app_path = format!("git@github.com:PrecisionNutrition/{}.git", name);
  let dest_path = format!("{}/{}", dest_root, name);
  let args = ["clone", &app_path, &dest_path];

  fs::create_dir_all(dest_root)?;

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
