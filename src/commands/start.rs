use std::path::Path;
use std::path::PathBuf;

use crate::devcontainers::config::Config;

pub fn run(dir: &Option<String>) -> Result<(), std::io::Error> {
    let directory = get_project_directory(dir)?;
    let config = get_configuration(&directory)?;

    println!("{:?}", config);

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
