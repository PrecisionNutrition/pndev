use failure::Error;
use self_update;

use crate::git;
use crate::shell;
// unix specific extensions for chmod
use std::os::unix::fs::PermissionsExt;

// see https://github.com/jaemk/self_update#usage
/// Performs a self update of pndev itself
pub fn run() -> Result<(), Error> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("PrecisionNutrition")
        .repo_name("pndev")
        .show_download_progress(true)
        .current_version(self_update::cargo_crate_version!())
        .build()?
        .update()?;

    println!("Update status: `{}`!", status.version());

    let exe_path = std::env::current_exe().expect("not executable");

    let metadata = std::fs::metadata(&exe_path)?;
    let mut perms = metadata.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&exe_path, perms)?;

    // ensure pndev is cloned
    shell::Shell::check_setup()?;

    // pull new docker configs
    git::update("pndev")?;

    Ok(())
}
