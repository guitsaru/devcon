use std::path::Path;
use std::path::PathBuf;

use crate::devcontainers::Devcontainer;

pub fn run(dir: &Option<String>) -> std::io::Result<()> {
    let directory = get_project_directory(dir)?;
    let devcontainer = Devcontainer::load(directory);
    devcontainer.run()?;

    Ok(())
}

fn get_project_directory(dir: &Option<String>) -> std::io::Result<PathBuf> {
    if let Some(path) = dir {
        let mut expanded = shellexpand::env(path).expect("Could not expand dir");

        Path::new(expanded.to_mut()).canonicalize()
    } else {
        std::env::current_dir()
    }
}
