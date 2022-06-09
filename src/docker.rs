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
    dockerfile: &Path,
    args: HashMap<String, String>,
    use_cache: bool,
) -> std::io::Result<String> {
    let mut command = Command::new("docker");
    command.arg("build");
    command.arg("-q");
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

    let hash = command.output()?.stdout;
    let str_hash = String::from_utf8(hash).unwrap();

    Ok(str_hash)
}

pub fn create(image_hash: &str, args: Vec<String>) -> std::io::Result<String> {
    let mut command = Command::new("docker");
    command.stderr(Stdio::inherit());
    command.arg("create");
    command.arg("-it");

    for arg in &args {
        command.arg(arg);
    }

    command.arg(image_hash.trim());
    command.arg("zsh");

    print_command(&command);

    let id = command.output()?.stdout;
    let str_id = String::from_utf8(id).unwrap().trim().to_string();

    Ok(str_id)
}

pub fn start(id: &str) -> std::io::Result<()> {
    let mut command = Command::new("docker");
    command.arg("start").arg(id);

    print_command(&command);
    command.status()?;

    Ok(())
}

pub fn stop(id: &str) -> std::io::Result<()> {
    let mut command = Command::new("docker");
    command.arg("stop").arg(id);

    print_command(&command);
    command.status()?;

    Ok(())
}

pub fn restart(id: &str) -> std::io::Result<()> {
    let mut command = Command::new("docker");
    command.arg("restart").arg(id);

    print_command(&command);
    command.status()?;

    Ok(())
}

pub fn attach(id: &str) -> std::io::Result<()> {
    let mut command = Command::new("docker");

    command.arg("attach").arg(id);

    print_command(&command);
    command.status()?;

    Ok(())
}

pub fn rm(id: &str) -> std::io::Result<()> {
    let mut command = Command::new("docker");
    command.arg("rm").arg(id);
    print_command(&command);
    command.status()?;

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

pub fn cp(name: &str, source: &Path, destination: &str) -> std::io::Result<()> {
    Command::new("docker")
        .arg("cp")
        .arg(source)
        .arg(format!("{}:{}", name, destination))
        .status()?;

    Ok(())
}

pub fn exec(name: &str, cmd: &str, user: &str, workspace_folder: &str) -> std::io::Result<()> {
    Command::new("docker")
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .arg("exec")
        .arg("-u")
        .arg(user)
        .arg("-w")
        .arg(workspace_folder)
        .arg(name)
        .arg("sh")
        .arg("-c")
        .arg(cmd)
        .status()?;
    Ok(())
}
