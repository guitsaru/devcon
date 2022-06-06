use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

pub fn build(dockerfile: &Path, args: HashMap<String, String>) -> std::io::Result<String> {
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

pub fn create(image_hash: &str, args: Vec<String>) -> std::io::Result<String> {
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

pub fn start(id: &str) -> std::io::Result<()> {
    println!("Starting docker container");

    Command::new("docker").arg("start").arg(id).status()?;

    Ok(())
}

pub fn stop(id: &str) -> std::io::Result<()> {
    println!("Stopping docker container");

    Command::new("docker").arg("stop").arg(id).status()?;

    Ok(())
}

pub fn attach(id: &str) -> std::io::Result<()> {
    println!("Attaching to docker container");
    Command::new("docker")
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .arg("attach")
        .arg(id)
        .status()?;

    Ok(())
}

pub fn rm(id: &str) -> std::io::Result<()> {
    Command::new("docker").arg("rm").arg(id).status()?;

    Ok(())
}

pub fn exists(name: &str) -> std::io::Result<bool> {
    let output = Command::new("docker")
        .arg("ps")
        .arg("-aq")
        .arg("--filter")
        .arg(format!("name={}", name))
        .output()?
        .stdout;

    let value = String::from_utf8(output).unwrap().trim().to_string();

    Ok(!value.is_empty())
}

pub fn running(name: &str) -> std::io::Result<bool> {
    let output = Command::new("docker")
        .arg("ps")
        .arg("-q")
        .arg("--filter")
        .arg(format!("name={}", name))
        .output()?
        .stdout;

    let value = String::from_utf8(output).unwrap().trim().to_string();

    Ok(!value.is_empty())
}
