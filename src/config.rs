use confy;
use dirs::home_dir;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    install_path: String,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            install_path: String::from("DEV/PN"),
        }
    }
}

// Linux:   /home/alice/.config/pndev/pndev.toml
// macOS:   /Users/Alice/Library/Preferences/rs.pndev.pndev/pndev.toml
const CONFIG_FILE_NAME: &str = "pndev";

impl Config {
    pub fn new() -> Self {
        info!("Loading/generating config file");
        let cfg: Config = confy::load(CONFIG_FILE_NAME).expect("Config load/write failed");
        cfg
    }

    pub fn home_path_str() -> String {
        let home_path = home_dir().unwrap();
        home_path.into_os_string().into_string().unwrap()
    }

    pub fn repo_path(&self) -> String {
        format!("{}/{}", Self::home_path_str(), self.install_path)
    }
}
