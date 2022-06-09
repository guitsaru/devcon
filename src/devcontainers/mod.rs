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
            if !docker::exists(self.docker_command(), name.as_str())? {
                self.create(use_cache)?;
            }

            if !docker::running(self.docker_command(), name.as_str())? {
                docker::start(self.docker_command(), name.as_str())?;
            }

            self.post_create()?;
            self.restart()?;

            docker::attach(self.docker_command(), name.as_str())?;

            if self.config.should_shutdown() {
                docker::stop(self.docker_command(), name.as_str())?;
            }
        } else {
            let name = self.config.safe_name();
            self.create(use_cache)?;
            self.post_create()?;

            docker_compose::attach(
                self.docker_command(),
                &name,
                self.config.service.clone().unwrap().as_str(),
                self.remote_user(),
                self.config.workspace_folder.clone().as_str(),
                "zsh",
            )?;

            if self.config.should_shutdown() {
                docker_compose::stop(self.docker_command(), &name)?;
            }
        }

        Ok(())
    }

    pub fn rebuild(&self, use_cache: bool) -> std::io::Result<()> {
        let name = self.config.safe_name();

        if self.config.is_docker() {
            if docker::exists(self.docker_command(), name.as_str())? {}

            docker::stop(self.docker_command(), &name)?;
            docker::rm(self.docker_command(), &name)?;

            self.run(false)
        } else {
            let name = self.config.safe_name();
            docker_compose::down(self.docker_command(), &name)?;

            self.run(use_cache)
        }
    }

    fn remote_user(&self) -> &str {
        self.config.remote_user.as_ref()
    }

    fn is_podman(&self) -> bool {
        self.docker_command() == "podman"
    }

    fn create_args(&self) -> Vec<String> {
        let mut create_args = self.config.create_args(self.directory.as_ref());

        if self.is_podman() {
            create_args.push("--userns=keep-id".to_string());
        }

        create_args
    }

    fn create(&self, use_cache: bool) -> std::io::Result<()> {
        let name = self.config.safe_name();

        if let Some(dockerfile) = self.dockerfile() {
            docker::build(
                self.docker_command(),
                &name,
                dockerfile.as_ref(),
                self.config.build_args(),
                use_cache,
            )?;

            docker::create(self.docker_command(), &name, self.create_args())?;
            docker::start(self.docker_command(), &name)?;
        } else if let Some(docker_compose_file) = self.docker_compose_file() {
            docker_compose::build(
                self.docker_command(),
                name.as_str(),
                &docker_compose_file,
                self.config.build_args(),
                use_cache,
            )?;

            docker_compose::start(self.docker_command(), name.as_str(), &docker_compose_file)?;
        }
        Ok(())
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

        Ok(())
    }

    fn restart(&self) -> std::io::Result<bool> {
        let name = self.config.safe_name();
        if self.config.is_docker() {
            docker::restart(self.docker_command(), &name)
        } else {
            docker_compose::restart(self.docker_command(), &name)
        }
    }

    fn exec(&self, command: &str) -> std::io::Result<bool> {
        let name = self.config.safe_name();
        let workspace_folder = self.config.workspace_folder.clone();
        let user = self.remote_user();

        if self.config.is_docker() {
            docker::exec(
                self.docker_command(),
                &name,
                command,
                &user,
                &workspace_folder,
            )
        } else {
            let service = self.config.service.clone().unwrap();

            docker_compose::exec(
                self.docker_command(),
                &name,
                &service,
                command,
                &user,
                &workspace_folder,
            )
        }
    }

    fn copy(&self, source: &Path, dest: &str) -> std::io::Result<bool> {
        if source.exists() {
            let name = self.config.safe_name();
            let destpath = PathBuf::from(dest);

            let basedir = destpath.parent().and_then(|p| p.to_str()).unwrap();

            let destination = if source.is_dir() { basedir } else { dest };

            if self.config.is_docker() {
                self.exec(format!("mkdir -p {}", basedir).as_str())?;
                docker::cp(self.docker_command(), &name, source, destination)
            } else {
                let service = self.config.service.clone().unwrap();

                self.exec(format!("mkdir -p {}", basedir).as_str())?;
                docker_compose::cp(self.docker_command(), &name, &service, source, dest)
            }
        } else {
            println!("Could not find file at {:?}", source);
            Ok(false)
        }
    }

    fn copy_dotfiles(&self) -> std::io::Result<()> {
        let settings = Settings::load();
        let homedir = if self.remote_user() == "root" {
            PathBuf::from("/root")
        } else {
            PathBuf::from("/home").join(self.remote_user())
        };

        for file in settings.dotfiles {
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
        let dest = format!("/home/{}/.gitconfig", self.remote_user());

        self.copy(&file, &dest)
    }

    fn docker_command(&self) -> String {
        "docker".to_string()
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
