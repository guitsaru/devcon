use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

pub fn build(dockerfile: &Path, args: HashMap<String, String>) -> Result<String, std::io::Error> {
    println!("Building docker container");

    let mut command = Command::new("docker");
    command.stderr(Stdio::inherit());
    command.arg("build");
    command.arg("-q");
    command.arg("-f");
    command.arg(dockerfile.to_str().unwrap());

    if !args.is_empty() {
        command.arg("--build-arg");
        for (key, value) in &args {
            command.arg(format!("{}={}", key, value));
        }
    }

    command.arg(".");

    let hash = command.output()?.stdout;
    let str_hash = String::from_utf8(hash).unwrap();

    Ok(str_hash)
}

pub fn create(image_hash: &str, args: Vec<String>) -> Result<String, std::io::Error> {
    println!("Creating docker container");

    let mut command = Command::new("docker");
    command.stderr(Stdio::inherit());
    command.arg("create");
    command.arg("-it");

    for arg in &args {
        command.arg(arg);
    }

    command.arg(image_hash.trim());
    command.arg("zsh");
    let id = command.output()?.stdout;
    let str_id = String::from_utf8(id).unwrap().trim().to_string();

    Ok(str_id)
}

pub fn attach(id: &str) -> Result<(), std::io::Error> {
    println!("Attaching to docker container");

    Command::new("docker")
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .arg("rm")
        .arg(id)
        .status()?;

    Ok(())
}

pub fn start(id: &str) -> Result<(), std::io::Error> {
    println!("Starting docker container");

    Command::new("docker")
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .arg("start")
        .arg("--attach")
        .arg("-i")
        .arg(id)
        .status()?;

    Ok(())
}

pub fn stop(id: &str) -> Result<(), std::io::Error> {
    println!("Stopping docker container");

    Command::new("docker")
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .arg("stop")
        .arg(id)
        .status()?;

    Ok(())
}
