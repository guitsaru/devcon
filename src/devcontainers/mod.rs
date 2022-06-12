pub mod config;

use crate::provider::docker::Docker;
use crate::provider::docker_compose::DockerCompose;
use crate::provider::podman::Podman;
use crate::provider::podman_compose::PodmanCompose;
use crate::provider::Provider;
use crate::settings::Settings;
use config::Config;
use std::path::Path;
use std::path::PathBuf;

pub struct Devcontainer {
    config: Config,
    provider: Box<dyn Provider>,
    settings: Settings,
}

impl Devcontainer {
    pub fn load(directory: PathBuf) -> Self {
        let file = directory.join(".devcontainer").join("devcontainer.json");
        let config = Config::parse(&file).expect("could not find devcontainer.json");
        let settings = Settings::load();
        let provider = build_provider(&directory, &settings, &config);

        Self {
            config: config.clone(),
            provider,
            settings,
        }
    }

    pub fn run(&self, use_cache: bool) -> std::io::Result<()> {
        let provider = &self.provider;

        self.create(use_cache)?;
        if !provider.running()? {
            provider.start()?;
        }

        self.post_create()?;
        provider.restart()?;
        provider.attach()?;

        if self.config.should_shutdown() {
            provider.stop()?;
        }

        Ok(())
    }

    pub fn rebuild(&self, use_cache: bool) -> std::io::Result<()> {
        let provider = &self.provider;
        if provider.exists()? {
            provider.stop()?;
            provider.rm()?;
        }

        self.run(use_cache)
    }

    fn create(&self, use_cache: bool) -> std::io::Result<()> {
        let provider = &self.provider;

        if !provider.exists()? {
            provider.build(use_cache)?;
            provider.create(self.create_args())?;
        }

        Ok(())
    }

    fn post_create(&self) -> std::io::Result<()> {
        let provider = &self.provider;

        if let Some(command) = self.config.on_create_command.clone() {
            provider.exec(command)?;
        }

        if let Some(command) = self.config.update_content_command.clone() {
            provider.exec(command)?;
        }

        if let Some(command) = self.config.post_create_command.clone() {
            provider.exec(command)?;
        }

        self.copy_gitconfig()?;
        self.copy_dotfiles()?;

        Ok(())
    }

    fn copy(&self, source: &Path, dest: &str) -> std::io::Result<bool> {
        if source.exists() {
            let provider = &self.provider;
            let destpath = PathBuf::from(dest);
            let basedir = destpath.parent().and_then(|p| p.to_str()).unwrap();
            let destination = if source.is_dir() { basedir } else { dest };

            provider.exec(format!("mkdir -p {}", basedir))?;
            provider.cp(
                source.to_string_lossy().to_string(),
                destination.to_string(),
            )
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found {:?}", source),
            ))
        }
    }

    fn copy_dotfiles(&self) -> std::io::Result<()> {
        let homedir = if self.config.remote_user == "root" {
            PathBuf::from("/root")
        } else {
            PathBuf::from("/home").join(&self.config.remote_user)
        };

        for file in &self.settings.dotfiles {
            let tilded = format!("~/{}", file);
            let expanded = shellexpand::tilde(&tilded).to_string();
            let source = PathBuf::from(expanded);
            let dest = homedir.join(file.clone());

            self.copy(&source, dest.to_str().unwrap())?;
        }

        Ok(())
    }

    fn copy_gitconfig(&self) -> std::io::Result<bool> {
        let path = shellexpand::tilde("~/.gitconfig").to_string();
        let file = PathBuf::from(path);
        let dest = format!("/home/{}/.gitconfig", self.config.remote_user);

        self.copy(&file, &dest)
    }

    pub fn create_args(&self) -> Vec<String> {
        let mut args = vec![];

        for port in &self.config.forward_ports {
            args.push("-p".to_string());
            let ports = format!("{}:{}", port, port);
            args.push(ports);
        }

        for (key, value) in &self.config.remote_env {
            args.push("-e".to_string());
            args.push(format!("{}={}", key, value));
        }

        let workspace_folder = self.config.workspace_folder.clone();
        args.push("-w".to_string());
        args.push(workspace_folder.clone());

        for arg in self.config.run_args.clone() {
            args.push(arg);
        }

        args
    }
}

fn build_provider(directory: &PathBuf, settings: &Settings, config: &Config) -> Box<dyn Provider> {
    match settings.provider {
        crate::settings::Provider::Docker => {
            if config.is_compose() {
                let composefile = directory
                    .join(".devcontainer")
                    .join(config.docker_compose_file.as_ref().unwrap());

                Box::new(DockerCompose {
                    build_args: config.build_args(),
                    directory: directory.to_str().map(|d| d.to_string()).unwrap(),
                    command: "docker".to_string(),
                    file: composefile.to_str().unwrap().to_string(),
                    name: config.safe_name(),
                    forward_ports: config.forward_ports.clone(),
                    run_args: config.run_args.clone(),
                    service: config.service.as_ref().unwrap().to_string(),
                    user: config.remote_user.clone(),
                    workspace_folder: config.workspace_folder.clone(),
                })
            } else {
                let dockerfile = directory
                    .join(".devcontainer")
                    .join(config.dockerfile().unwrap());

                Box::new(Docker {
                    build_args: config.build_args(),
                    directory: directory.to_str().map(|d| d.to_string()).unwrap(),
                    command: "docker".to_string(),
                    file: dockerfile.to_str().unwrap().to_string(),
                    name: config.safe_name(),
                    run_args: config.run_args.clone(),
                    user: config.remote_user.clone(),
                    workspace_folder: config.workspace_folder.clone(),
                })
            }
        }
        crate::settings::Provider::Podman => {
            if config.is_compose() {
                let composefile = directory
                    .join(".devcontainer")
                    .join(config.docker_compose_file.as_ref().unwrap());

                Box::new(PodmanCompose {
                    build_args: config.build_args(),
                    directory: directory.to_str().map(|d| d.to_string()).unwrap(),
                    command: "podman-compose".to_string(),
                    file: composefile.to_str().unwrap().to_string(),
                    name: config.safe_name(),
                    podman_command: "podman".to_string(),
                    run_args: config.run_args.clone(),
                    service: config.service.as_ref().unwrap().to_string(),
                    user: config.remote_user.clone(),
                    workspace_folder: config.workspace_folder.clone(),
                })
            } else {
                let dockerfile = directory
                    .join(".devcontainer")
                    .join(config.dockerfile().unwrap());

                Box::new(Podman {
                    build_args: config.build_args(),
                    directory: directory.to_str().map(|d| d.to_string()).unwrap(),
                    command: "podman".to_string(),
                    file: dockerfile.to_str().unwrap().to_string(),
                    name: config.safe_name(),
                    run_args: config.run_args.clone(),
                    user: config.remote_user.clone(),
                    workspace_folder: config.workspace_folder.clone(),
                })
            }
        }
    }
}
