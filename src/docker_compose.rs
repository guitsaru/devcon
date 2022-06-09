use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

fn print_command(command: &Command) {
    let exec = command.get_program();
    let args: Vec<&str> = command
        .get_args()
        .map(|arg| arg.to_str().unwrap())
        .collect();

    println!("{} {}", exec.to_str().unwrap(), args.join(" "));
}

pub fn build(
    name: &str,
    docker_compose_file: &Path,
    args: HashMap<String, String>,
    use_cache: bool,
) -> std::io::Result<()> {
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

    print_command(&command);

    command.status()?;

    Ok(())
}

pub fn start(name: &str, docker_compose_file: &Path) -> std::io::Result<()> {
    let mut command = Command::new("docker");
    command
        .arg("compose")
        .arg("-f")
        .arg(docker_compose_file.to_str().unwrap())
        .arg("-p")
        .arg(name)
        .arg("up")
        .arg("--detach");

    print_command(&command);

    command.status()?;

    Ok(())
}

pub fn down(name: &str) -> std::io::Result<()> {
    let mut command = Command::new("docker");
    command
        .arg("compose")
        .arg("-p")
        .arg(name)
        .arg("down")
        .arg("--remove-orphans")
        .arg("--rmi")
        .arg("all");

    print_command(&command);
    command.status()?;

    Ok(())
}

pub fn stop(name: &str) -> std::io::Result<()> {
    let mut command = Command::new("docker");
    command.arg("compose").arg("-p").arg(name).arg("stop");
    print_command(&command);
    command.status()?;

    Ok(())
}

pub fn restart(name: &str) -> std::io::Result<()> {
    let mut command = Command::new("docker");
    command.arg("compose").arg("-p").arg(name).arg("restart");
    print_command(&command);
    command.status()?;

    Ok(())
}

pub fn attach(
    name: &str,
    service: &str,
    user: &str,
    workspace_folder: &str,
    cmd: &str,
) -> std::io::Result<()> {
    let mut command = Command::new("docker");
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
    command.status()?;

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
