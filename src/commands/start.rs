use crate::docker;
use std::path::Path;
use std::path::PathBuf;

use crate::devcontainers::config::Config;

pub fn run(dir: &Option<String>) -> Result<(), std::io::Error> {
    let directory = get_project_directory(dir)?;
    let config = get_configuration(&directory)?;
    let dockerfile = directory
        .join(".devcontainer")
        .join(config.dockerfile().unwrap());

    let hash = docker::build(dockerfile.as_ref(), config.build_args())?;
    let id = docker::create(hash.as_ref(), config.create_args(directory.as_ref()))?;
    println!("ID: {}", id);
    docker::start(id.as_ref())?;
    docker::stop(id.as_ref())?;

    Ok(())
}

fn get_project_directory(dir: &Option<String>) -> Result<PathBuf, std::io::Error> {
    if let Some(path) = dir {
        let mut expanded = shellexpand::env(path).expect("Could not expand dir");

        Path::new(expanded.to_mut()).canonicalize()
    } else {
        std::env::current_dir()
    }
}

fn get_configuration(dir: &Path) -> Result<Config, std::io::Error> {
    let file = dir.join(".devcontainer/devcontainer.json");
    let config = Config::parse(&file)?;

    Ok(config)
}
