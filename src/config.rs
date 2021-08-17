use dirs::home_dir;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    install_path: String,
    docker_compose_path: Option<String>,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            install_path: String::from("DEV/PN"),
            docker_compose_path: None,
        }
    }
}

const CONFIG_FILE_NAME: &str = ".pndev_config";

impl Config {
    pub fn new() -> Self {
        let path = format!("{}/{}", Self::home_path_str(), CONFIG_FILE_NAME);
        info!("Loading config from {}.toml", path);
        let cfg: Self = confy::load(&path).expect("Config load/write failed");

        cfg
    }

    pub fn home_path_str() -> String {
        let home_path = home_dir().unwrap();
        home_path.into_os_string().into_string().unwrap()
    }

    pub fn repo_path(&self) -> String {
        format!("{}/{}", Self::home_path_str(), self.install_path)
    }

    pub fn docker_compose_path(&self) -> String {
        match &self.docker_compose_path {
            Some(path) => {
                format!("{}/{}", Self::home_path_str(), path)
            }
            None => {
                format!(
                    "{}/{}/pndev/catalog/docker-compose.yml",
                    Self::home_path_str(),
                    self.install_path
                )
            }
        }
    }
}
