use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

pub fn build(
    name: &str,
    docker_compose_file: &Path,
    args: HashMap<String, String>,
    use_cache: bool,
) -> std::io::Result<()> {
    println!("Building docker compose");

    let mut command = Command::new("docker");
    command.arg("compose");
    command.arg("-f");
    command.arg(docker_compose_file.to_str().unwrap());
    command.arg("-p");
    command.arg(name);
    command.arg("build");
    command.arg("--pull");
    command.arg("-q");

    if !use_cache {
        command.arg("--no-cache");
    }

    if !args.is_empty() {
        command.arg("--build-arg");
        for (key, value) in &args {
            command.arg(format!("{}={}", key, value));
        }
    }

    command.status()?;

    Ok(())
}

pub fn start(name: &str, docker_compose_file: &Path) -> std::io::Result<()> {
    println!("Starting docker compose");

    Command::new("docker")
        .arg("compose")
        .arg("-f")
        .arg(docker_compose_file.to_str().unwrap())
        .arg("-p")
        .arg(name)
        .arg("up")
        .arg("--detach")
        .status()?;

    Ok(())
}

pub fn stop(name: &str) -> std::io::Result<()> {
    println!("Starting docker compose");

    Command::new("docker")
        .arg("compose")
        .arg("-p")
        .arg(name)
        .arg("down")
        .status()?;

    Ok(())
}

pub fn restart(name: &str) -> std::io::Result<()> {
    Command::new("docker")
        .arg("compose")
        .arg("-p")
        .arg(name)
        .arg("restart")
        .status()?;

    Ok(())
}

pub fn attach(
    name: &str,
    service: &str,
    user: &str,
    workspace_folder: &str,
    command: &str,
) -> std::io::Result<()> {
    println!("Starting docker compose");

    Command::new("docker")
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .arg("compose")
        .arg("-p")
        .arg(name)
        .arg("exec")
        .arg("-u")
        .arg(user)
        .arg("-w")
        .arg(workspace_folder)
        .arg(service)
        .arg(command)
        .status()?;

    Ok(())
}

pub fn cp(name: &str, service: &str, source: &Path, destination: &str) -> std::io::Result<()> {
    Command::new("docker")
        .arg("compose")
        .arg("-p")
        .arg(name)
        .arg("cp")
        .arg(source)
        .arg(format!("{}:{}", service, destination))
        .status()?;

    Ok(())
}

pub fn exec(
    name: &str,
    service: &str,
    cmd: &str,
    user: &str,
    workspace_folder: &str,
) -> std::io::Result<()> {
    Command::new("docker")
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .arg("compose")
        .arg("-p")
        .arg(name)
        .arg("exec")
        .arg("-u")
        .arg(user)
        .arg("-w")
        .arg(workspace_folder)
        .arg(service)
        .arg("sh")
        .arg("-c")
        .arg(cmd)
        .status()?;

    Ok(())
}
