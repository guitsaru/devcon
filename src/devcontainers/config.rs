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
    forward_ports: Vec<u16>,
    pub on_create_command: Option<String>,
    pub update_content_command: Option<String>,
    pub post_create_command: Option<String>,
    #[serde(default = "default_remote_user")]
    pub remote_user: String,
    #[serde(default)]
    pub run_args: Vec<String>,
    #[serde(default)]
    remote_env: HashMap<String, String>,
    docker_compose_file: Option<String>,
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

    pub fn docker_compose_file(&self) -> Option<String> {
        self.docker_compose_file.clone()
    }

    pub fn build_args(&self) -> HashMap<String, String> {
        self.build.clone().map(|b| b.args).unwrap_or_default()
    }

    pub fn create_args(&self, workspace: &Path) -> Vec<String> {
        let mut args = vec![
            "--name".to_string(),
            self.safe_name(),
            "-u".to_string(),
            self.remote_user.clone(),
        ];

        let forward_ports = self.forward_ports.clone();
        if !forward_ports.is_empty() {
            args.push("-p".to_string());
            for port in forward_ports {
                let ports = format!("{}:{}", port, port);
                args.push(ports);
            }
        }

        if !self.remote_env.is_empty() {
            args.push("-e".to_string());
            for (key, value) in &self.remote_env {
                args.push(format!("{}={}", key, value));
            }
        }

        let workspace_folder = self.workspace_folder.clone();
        args.push("-w".to_string());
        args.push(workspace_folder.clone());

        args.push("--mount".to_string());
        args.push(format!(
            "type=bind,source={},target={}",
            workspace.to_str().unwrap(),
            workspace_folder
        ));

        for arg in self.run_args.clone() {
            args.push(arg);
        }

        args
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

    pub fn is_docker(&self) -> bool {
        self.docker_compose_file.is_none()
    }
}

fn default_remote_user() -> String {
    "root".to_string()
}

fn default_workspace_folder() -> String {
    "/workspace".to_string()
}
