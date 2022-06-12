extern crate directories;
use directories::ProjectDirs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Docker,
    Podman,
}

impl Default for Provider {
    fn default() -> Self {
        Self::Docker
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Settings {
    pub dotfiles: Vec<String>,
    #[serde(default)]
    pub provider: Provider,
}

impl Settings {
    pub fn load() -> Self {
        if let Some(dirs) = ProjectDirs::from("com", "Big Refactor", "devcon") {
            let dir = dirs.config_dir();
            let file = dir.join("config.toml");

            if file.is_file() {
                let contents = std::fs::read_to_string(file).unwrap();
                toml::from_str(&contents).unwrap()
            } else {
                Self::default()
            }
        } else {
            Self::default()
        }
    }
}
