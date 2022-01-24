use std::path::Path;
use std::path::PathBuf;

pub fn run(dir: &Option<String>) {
    let directory = get_project_directory(dir).expect("No project directory.");
    println!("{:?}", directory);
}

fn get_project_directory(dir: &Option<String>) -> Result<PathBuf, std::io::Error> {
    if let Some(path) = dir {
        let mut expanded = shellexpand::env(path).expect("Could not expand dir");

        Path::new(expanded.to_mut()).canonicalize()
    } else {
        std::env::current_dir()
    }
}
