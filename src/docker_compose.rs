use colored::Colorize;
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

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
    name: &str,
    docker_compose_file: &Path,
    args: HashMap<String, String>,
    use_cache: bool,
) -> std::io::Result<bool> {
    let mut command = Command::new(docker_command);
    command.arg("compose");
    command.arg("-f");
    command.arg(docker_compose_file.to_str().unwrap());
    command.arg("-p");
    command.arg(name);
    command.arg("build");
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

    print_command(&command);

    Ok(command.status()?.success())
}

pub fn start(
    docker_command: String,
    name: &str,
    docker_compose_file: &Path,
) -> std::io::Result<bool> {
    let mut command = Command::new(docker_command);
    command
        .arg("compose")
        .arg("-f")
        .arg(docker_compose_file.to_str().unwrap())
        .arg("-p")
        .arg(name)
        .arg("up")
        .arg("--detach");

    print_command(&command);

    Ok(command.status()?.success())
}

pub fn down(docker_command: String, name: &str) -> std::io::Result<bool> {
    let mut command = Command::new(docker_command);
    command
        .arg("compose")
        .arg("-p")
        .arg(name)
        .arg("down")
        .arg("--remove-orphans")
        .arg("--rmi")
        .arg("all");

    print_command(&command);

    Ok(command.status()?.success())
}

pub fn stop(docker_command: String, name: &str) -> std::io::Result<bool> {
    let mut command = Command::new(docker_command);
    command.arg("compose").arg("-p").arg(name).arg("stop");
    print_command(&command);

    Ok(command.status()?.success())
}

pub fn restart(docker_command: String, name: &str) -> std::io::Result<bool> {
    let mut command = Command::new(docker_command);
    command.arg("compose").arg("-p").arg(name).arg("restart");
    print_command(&command);

    Ok(command.status()?.success())
}

pub fn attach(
    docker_command: String,
    name: &str,
    service: &str,
    user: &str,
    workspace_folder: &str,
    cmd: &str,
) -> std::io::Result<bool> {
    let mut command = Command::new(docker_command);
    command
        .arg("compose")
        .arg("-p")
        .arg(name)
        .arg("exec")
        .arg("-u")
        .arg(user)
        .arg("-w")
        .arg(workspace_folder)
        .arg(service)
        .arg(cmd);
    print_command(&command);

    Ok(command.status()?.success())
}

pub fn cp(
    docker_command: String,
    name: &str,
    service: &str,
    source: &Path,
    destination: &str,
) -> std::io::Result<bool> {
    let mut command = Command::new(docker_command);
    command
        .arg("compose")
        .arg("-p")
        .arg(name)
        .arg("cp")
        .arg(source)
        .arg(format!("{}:{}", service, destination));

    Ok(command.status()?.success())
}

pub fn exec(
    docker_command: String,
    name: &str,
    service: &str,
    cmd: &str,
    user: &str,
    workspace_folder: &str,
) -> std::io::Result<bool> {
    let mut command = Command::new(docker_command);
    command
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
        .arg(cmd);

    Ok(command.status()?.success())
}
