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
        if let Some(dockerfile) = self.dockerfile() {
            let hash = docker::build(dockerfile.as_ref(), self.config.build_args())?;
            let id = docker::create(
                hash.as_ref(),
                self.config.create_args(self.directory.as_ref()),
            )?;
            docker::start(id.as_ref())?;
            docker::stop(id.as_ref())?;
        }

        Ok(())
    }

    pub fn dockerfile(&self) -> Option<PathBuf> {
        self.config
            .dockerfile()
            .map(|dockerfile| self.directory.join(".devcontainer").join(dockerfile))
    }
}
