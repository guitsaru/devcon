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

            self.post_create()?;

            docker::attach(name.as_str())?;

            if self.config.should_shutdown() {
                docker::stop(name.as_str())?;
            }
        } else {
            let name = self.config.safe_name();
            self.create()?;
            self.post_create()?;

            docker_compose::attach(
                &name,
                self.config.service.clone().unwrap().as_str(),
                self.config.remote_user.clone().as_str(),
                self.config.workspace_folder.clone().as_str(),
                "zsh",
            )?;

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
            Ok("".to_string())
        } else {
            Ok("".to_string())
        }
    }

    fn post_create(&self) -> std::io::Result<()> {
        self.copy_gitconfig()?;

        if let Some(command) = self.config.on_create_command.clone() {
            let name = self.config.safe_name();

            if self.config.is_docker() {
                docker::exec(&name, &command)?;
            } else {
                let service = self.config.service.clone().unwrap();
                let workspace_folder = self.config.workspace_folder.clone();
                let user = self.config.remote_user.clone();

                docker_compose::exec(&name, &service, &command, &user, &workspace_folder)?;
            }
        }

        if let Some(command) = self.config.update_content_command.clone() {
            let name = self.config.safe_name();

            if self.config.is_docker() {
                docker::exec(&name, &command)?;
            } else {
                let service = self.config.service.clone().unwrap();
                let workspace_folder = self.config.workspace_folder.clone();
                let user = self.config.remote_user.clone();

                docker_compose::exec(&name, &service, &command, &user, &workspace_folder)?;
            }
        }

        if let Some(command) = self.config.post_create_command.clone() {
            let name = self.config.safe_name();

            if self.config.is_docker() {
                docker::exec(&name, &command)?;
            } else {
                let service = self.config.service.clone().unwrap();
                let workspace_folder = self.config.workspace_folder.clone();
                let user = self.config.remote_user.clone();

                docker_compose::exec(&name, &service, &command, &user, &workspace_folder)?;
            }
        }

        Ok(())
    }

    fn copy_gitconfig(&self) -> std::io::Result<()> {
        let path = shellexpand::tilde("~/.gitconfig").to_string();
        let file = PathBuf::from(path);

        if file.is_file() {
            let name = self.config.safe_name();
            let dest = format!("/home/{}/.gitconfig", self.config.remote_user.clone());

            if self.config.is_docker() {
                docker::cp(&name, &file, &dest)
            } else {
                docker_compose::cp(&name, &self.config.service.clone().unwrap(), &file, &dest)
            }
        } else {
            Ok(())
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
