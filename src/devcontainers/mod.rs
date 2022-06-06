pub mod config;

use crate::docker;
use crate::docker_compose;
use crate::settings::Settings;
use config::Config;
use std::path::Path;
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

    pub fn run(&self, use_cache: bool) -> std::io::Result<()> {
        let name = self.config.safe_name();

        if self.config.is_docker() {
            if !docker::exists(name.as_str())? {
                self.create(use_cache)?;
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
            self.create(use_cache)?;
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

    pub fn rebuild(&self, use_cache: bool) -> std::io::Result<()> {
        let name = self.config.safe_name();

        if self.config.is_docker() {
            if docker::exists(name.as_str())? {}

            docker::stop(&name)?;
            docker::rm(&name)?;

            self.run(false)
        } else {
            let name = self.config.safe_name();
            docker_compose::stop(&name)?;

            self.run(use_cache)
        }
    }

    fn create(&self, use_cache: bool) -> std::io::Result<String> {
        if let Some(dockerfile) = self.dockerfile() {
            let hash = docker::build(dockerfile.as_ref(), self.config.build_args(), use_cache)?;
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
                use_cache,
            )?;

            docker_compose::start(name.as_str(), &docker_compose_file)?;
            Ok("".to_string())
        } else {
            Ok("".to_string())
        }
    }

    fn post_create(&self) -> std::io::Result<()> {
        if let Some(command) = self.config.on_create_command.clone() {
            self.exec(&command)?;
        }

        if let Some(command) = self.config.update_content_command.clone() {
            self.exec(&command)?;
        }

        if let Some(command) = self.config.post_create_command.clone() {
            self.exec(&command)?;
        }

        self.copy_gitconfig()?;
        self.copy_dotfiles()?;
        self.restart()?;

        Ok(())
    }

    fn restart(&self) -> std::io::Result<()> {
        let name = self.config.safe_name();
        if self.config.is_docker() {
            docker::restart(&name)
        } else {
            docker_compose::restart(&name)
        }
    }

    fn exec(&self, command: &str) -> std::io::Result<()> {
        let name = self.config.safe_name();
        let workspace_folder = self.config.workspace_folder.clone();
        let user = self.config.remote_user.clone();

        if self.config.is_docker() {
            docker::exec(&name, command, &user, &workspace_folder)
        } else {
            let service = self.config.service.clone().unwrap();

            docker_compose::exec(&name, &service, command, &user, &workspace_folder)
        }
    }

    fn copy(&self, source: &Path, dest: &str) -> std::io::Result<()> {
        if source.exists() {
            let name = self.config.safe_name();
            let destpath = PathBuf::from(dest);
            let basedir = destpath.parent().and_then(|p| p.to_str()).unwrap();

            if self.config.is_docker() {
                self.exec(format!("mkdir -p {}", basedir).as_str())?;
                docker::cp(&name, source, dest)
            } else {
                let service = self.config.service.clone().unwrap();

                self.exec(format!("mkdir -p {}", basedir).as_str())?;
                docker_compose::cp(&name, &service, source, dest)
            }
        } else {
            println!("not a file");
            Ok(())
        }
    }

    fn copy_dotfiles(&self) -> std::io::Result<()> {
        let settings = Settings::load();

        for file in settings.dotfiles {
            let tilded = format!("~/{}", file);
            let expanded = shellexpand::tilde(&tilded).to_string();
            let source = PathBuf::from(expanded);
            let dest = PathBuf::from("/home")
                .join(self.config.remote_user.clone())
                .join(file.clone());

            self.copy(&source, dest.to_str().unwrap())?;
        }

        Ok(())
    }

    fn copy_gitconfig(&self) -> std::io::Result<()> {
        let path = shellexpand::tilde("~/.gitconfig").to_string();
        let file = PathBuf::from(path);
        let dest = format!("/home/{}/.gitconfig", self.config.remote_user.clone());

        self.copy(&file, &dest)
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
