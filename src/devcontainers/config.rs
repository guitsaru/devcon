use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
enum ShutdownAction {
    None,
    StopContainer,
    StopCompose,
}

impl Default for ShutdownAction {
    fn default() -> Self {
        Self::StopContainer
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    name: String,
    pub build: Option<Build>,
    #[serde(default)]
    pub forward_ports: Vec<u16>,
    pub on_create_command: Option<String>,
    pub update_content_command: Option<String>,
    pub post_create_command: Option<String>,
    #[serde(default = "default_remote_user")]
    pub remote_user: String,
    #[serde(default)]
    pub run_args: Vec<String>,
    #[serde(default)]
    pub remote_env: HashMap<String, String>,
    pub docker_compose_file: Option<String>,
    pub service: Option<String>,
    #[serde(default = "default_workspace_folder")]
    pub workspace_folder: String,
    #[serde(default)]
    shutdown_action: ShutdownAction,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Build {
    pub dockerfile: Option<String>,
    #[serde(default)]
    pub args: HashMap<String, String>,
}

impl Config {
    pub fn parse(file: &Path) -> Result<Config, std::io::Error> {
        let contents = std::fs::read_to_string(file)?;
        let config: Config = json5::from_str(&contents).unwrap();

        Ok(config)
    }

    pub fn dockerfile(&self) -> Option<String> {
        self.build.clone().and_then(|b| b.dockerfile)
    }

    pub fn build_args(&self) -> HashMap<String, String> {
        self.build.clone().map(|b| b.args).unwrap_or_default()
    }

    pub fn safe_name(&self) -> String {
        let name = self
            .name
            .to_lowercase()
            .replace(' ', "-")
            .trim()
            .to_string();

        format!("devcon-{}", name)
    }

    pub fn should_shutdown(&self) -> bool {
        !matches!(self.shutdown_action, ShutdownAction::None)
    }

    pub fn is_compose(&self) -> bool {
        self.docker_compose_file.is_some()
    }
}

fn default_remote_user() -> String {
    "root".to_string()
}

fn default_workspace_folder() -> String {
    "/workspace".to_string()
}
