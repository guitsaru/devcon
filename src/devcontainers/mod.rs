pub mod config;

use crate::docker;
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
        } else {
            Ok("".to_string())
        }
    }

    fn dockerfile(&self) -> Option<PathBuf> {
        self.config
            .dockerfile()
            .map(|dockerfile| self.directory.join(".devcontainer").join(dockerfile))
    }
}
