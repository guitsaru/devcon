pub mod config;

use crate::docker;
use crate::docker_compose;
use config::Config;
use std::path::PathBuf;

pub struct Devcontainer {
    config: Config,
    directory: PathBuf,
}

impl Devcontainer {
    pub fn load(directory: PathBuf) -> Self {
        let file = directory.join(".devcontainer").join("devcontainer.json");
        let config = Config::parse(&file).expect("could not find devcontainer.json");

        Self { config, directory }
    }

    pub fn run(&self) -> std::io::Result<()> {
        let name = self.config.safe_name();

        if self.config.is_docker() {
            if !docker::exists(name.as_str())? {
                self.create()?;
            }

            if !docker::running(name.as_str())? {
                docker::start(name.as_str())?;
            }

            docker::attach(name.as_str())?;

            if self.config.should_shutdown() {
                docker::stop(name.as_str())?;
            }
        } else {
            let name = self.config.safe_name();
            self.create()?;

            if self.config.should_shutdown() {
                docker_compose::stop(&name)?;
            }
        }

        Ok(())
    }

    fn create(&self) -> std::io::Result<String> {
        if let Some(dockerfile) = self.dockerfile() {
            let hash = docker::build(dockerfile.as_ref(), self.config.build_args())?;
            let id = docker::create(
                hash.as_ref(),
                self.config.create_args(self.directory.as_ref()),
            )?;
            docker::start(id.as_ref())?;

            Ok(id)
        } else if let Some(docker_compose_file) = self.docker_compose_file() {
            let name = self.config.safe_name();

            docker_compose::build(
                name.as_str(),
                &docker_compose_file,
                self.config.build_args(),
            )?;

            docker_compose::start(name.as_str(), &docker_compose_file)?;
            docker_compose::attach(
                &name,
                self.config.service.clone().unwrap().as_str(),
                self.config.remote_user.clone().as_str(),
                self.config.workspace_folder.clone().as_str(),
                "zsh",
            )?;
            Ok("".to_string())
        } else {
            Ok("".to_string())
        }
    }

    fn dockerfile(&self) -> Option<PathBuf> {
        self.config
            .dockerfile()
            .map(|dockerfile| self.directory.join(".devcontainer").join(dockerfile))
    }

    fn docker_compose_file(&self) -> Option<PathBuf> {
        self.config
            .docker_compose_file()
            .map(|docker_compose_file| {
                self.directory
                    .join(".devcontainer")
                    .join(docker_compose_file)
            })
    }
}
