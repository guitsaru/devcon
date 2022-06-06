use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    name: String,
    pub build: Build,
    #[serde(default)]
    forward_ports: Vec<u16>,
    post_create_command: Option<String>,
    #[serde(default = "default_remote_user")]
    remote_user: String,
    #[serde(default)]
    run_args: Vec<String>,
    #[serde(default)]
    remote_env: HashMap<String, String>,
    docker_compose_file: Option<String>,
    service: Option<String>,
    #[serde(default = "default_workspace_folder")]
    workspace_folder: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Build {
    pub dockerfile: Option<String>,
    #[serde(default)]
    pub args: HashMap<String, String>,
}

impl Config {
    pub fn parse(file: &Path) -> Result<Config, std::io::Error> {
        let contents = std::fs::read_to_string(file)?;
        let config: Config =
            json5::from_str(&contents).unwrap_or_else(|_| panic!("Could not parse {:?}", file));

        Ok(config)
    }

    pub fn dockerfile(&self) -> Option<String> {
        self.build.dockerfile.clone()
    }

    pub fn build_args(&self) -> HashMap<String, String> {
        self.build.args.clone()
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

    fn safe_name(&self) -> String {
        self.name
            .to_lowercase()
            .replace(' ', "-")
            .trim()
            .to_string()
    }
}

fn default_remote_user() -> String {
    "root".to_string()
}

fn default_workspace_folder() -> String {
    "/workspace".to_string()
}
