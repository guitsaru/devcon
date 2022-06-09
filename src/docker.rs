use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

use colored::Colorize;

fn print_command(command: &Command) {
    let exec = command.get_program();
    let args: Vec<&str> = command
        .get_args()
        .map(|arg| arg.to_str().unwrap())
        .collect();

    let output = format!("{} {}", exec.to_str().unwrap(), args.join(" "));
    println!("");
    println!("{}", output.bold().blue());
}

pub fn build(
    docker_command: String,
    name: &String,
    dockerfile: &Path,
    args: HashMap<String, String>,
    use_cache: bool,
) -> std::io::Result<bool> {
    let mut command = Command::new(docker_command.clone());
    command.arg("build");
    command.arg("-t");
    command.arg(name);
    command.arg("-f");
    command.arg(dockerfile.to_str().unwrap());

    if !use_cache {
        command.arg("--no-cache");
    }

    if !args.is_empty() {
        for (key, value) in &args {
            command.arg("--build-arg");
            command.arg(format!("{}={}", key, value));
        }
    }

    command.arg(".");

    print_command(&command);

    Ok(command.status()?.success())
}

pub fn create(docker_command: String, name: &str, args: Vec<String>) -> std::io::Result<bool> {
    let mut command = Command::new(docker_command);
    command.arg("create");
    command.arg("-it");

    for arg in &args {
        command.arg(arg);
    }

    command.arg(name);
    command.arg("zsh");

    print_command(&command);

    Ok(command.status()?.success())
}

pub fn start(docker_command: String, id: &str) -> std::io::Result<bool> {
    let mut command = Command::new(docker_command);
    command.arg("start").arg(id);

    print_command(&command);

    Ok(command.status()?.success())
}

pub fn stop(docker_command: String, id: &str) -> std::io::Result<bool> {
    let mut command = Command::new(docker_command);
    command.arg("stop").arg(id);

    print_command(&command);

    Ok(command.status()?.success())
}

pub fn restart(docker_command: String, id: &str) -> std::io::Result<bool> {
    let mut command = Command::new(docker_command);
    command.arg("restart").arg(id);

    print_command(&command);

    Ok(command.status()?.success())
}

pub fn attach(docker_command: String, id: &str) -> std::io::Result<bool> {
    let mut command = Command::new(docker_command);

    command.arg("attach").arg(id);

    print_command(&command);

    Ok(command.status()?.success())
}

pub fn rm(docker_command: String, id: &str) -> std::io::Result<bool> {
    let mut command = Command::new(docker_command);
    command.arg("rm").arg(id);
    print_command(&command);

    Ok(command.status()?.success())
}

pub fn exists(docker_command: String, name: &str) -> std::io::Result<bool> {
    let output = Command::new(docker_command)
        .arg("ps")
        .arg("-aq")
        .arg("--filter")
        .arg(format!("name={}", name))
        .output()?
        .stdout;

    let value = String::from_utf8(output).unwrap().trim().to_string();

    Ok(!value.is_empty())
}

pub fn running(docker_command: String, name: &str) -> std::io::Result<bool> {
    let output = Command::new(docker_command)
        .arg("ps")
        .arg("-q")
        .arg("--filter")
        .arg(format!("name={}", name))
        .output()?
        .stdout;

    let value = String::from_utf8(output).unwrap().trim().to_string();

    Ok(!value.is_empty())
}

pub fn cp(
    docker_command: String,
    name: &str,
    source: &Path,
    destination: &str,
) -> std::io::Result<bool> {
    let mut command = Command::new(docker_command);
    command
        .arg("cp")
        .arg(source)
        .arg(format!("{}:{}", name, destination));

    print_command(&command);

    Ok(command.status()?.success())
}

pub fn exec(
    docker_command: String,
    name: &str,
    cmd: &str,
    user: &str,
    workspace_folder: &str,
) -> std::io::Result<bool> {
    let mut command = Command::new(docker_command);
    command
        .arg("exec")
        .arg("-u")
        .arg(user)
        .arg("-w")
        .arg(workspace_folder)
        .arg(name)
        .arg("sh")
        .arg("-c")
        .arg(cmd);

    print_command(&command);

    Ok(command.status()?.success())
}

pub fn unshare(docker_command: String, cmd: &str) -> std::io::Result<bool> {
    let mut command = Command::new(docker_command);
    command.arg("unshare").arg(cmd);

    print_command(&command);

    Ok(command.status()?.success())
}
