use failure::Error;
use self_update;

// unix specific extensions for chmod
use std::os::unix::fs::PermissionsExt;

// see https://github.com/jaemk/self_update#usage
/// Performs a self update of pndev itself
pub fn run() -> Result<(), Error> {
    let token = std::env::var("DOWNLOAD_AUTH_TOKEN")
        .expect("DOWNLOAD_AUTH_TOKEN needs to the best in your environment");
    let status = self_update::backends::github::Update::configure()
        .repo_owner("PrecisionNutrition")
        .repo_name("pndev")
        .bin_name("pndev-linux-amd64")
        .auth_token(&token)
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

    Ok(())
}