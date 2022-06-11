use std::collections::HashMap;
use std::io::Result;
use std::process::Command;

use super::print_command;
use super::Provider;

#[derive(Debug)]
pub struct PodmanCompose {
    pub build_args: HashMap<String, String>,
    pub command: String,
    pub directory: String,
    pub file: String,
    pub name: String,
    pub run_args: Vec<String>,
    pub service: String,
    pub user: String,
    pub workspace_folder: String,
}

impl Provider for PodmanCompose {
    fn build(&self, use_cache: bool) -> Result<bool> {
        let mut command = Command::new(&self.command);
        command
            .arg("-f")
            .arg(&self.file)
            .arg("-p")
            .arg(&self.name)
            .arg("build");

        if !use_cache {
            command.arg("--no-cache");
        }

        for (key, value) in &self.build_args {
            command.arg("--build-arg").arg(format!("{}={}", key, value));
        }

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn create(&self) -> Result<bool> {
        Ok(true)
    }

    fn start(&self) -> Result<bool> {
        let mut command = Command::new(&self.command);
        command
            .arg("-f")
            .arg(&self.file)
            .arg("-p")
            .arg(&self.name)
            .arg("up")
            .arg("--detach");

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn stop(&self) -> Result<bool> {
        let mut command = Command::new(&self.command);
        command
            .arg("-f")
            .arg(&self.file)
            .arg("-p")
            .arg(&self.name)
            .arg("stop");

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn restart(&self) -> Result<bool> {
        let mut command = Command::new(&self.command);
        command
            .arg("-f")
            .arg(&self.file)
            .arg("-p")
            .arg(&self.name)
            .arg("restart");

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn attach(&self) -> Result<bool> {
        let mut command = Command::new(&self.command);
        command
            .arg("-f")
            .arg(&self.file)
            .arg("-p")
            .arg(&self.name)
            .arg("exec")
            .arg("-u")
            .arg(&self.user)
            .arg("-w")
            .arg(&self.workspace_folder)
            .arg(&self.service)
            .arg("zsh");

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn rm(&self) -> Result<bool> {
        let mut command = Command::new(&self.command);
        command
            .arg("-f")
            .arg(&self.file)
            .arg("-p")
            .arg(&self.name)
            .arg("down")
            .arg("--remove-orphans")
            .arg("--rmi")
            .arg("all");

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn exists(&self) -> Result<bool> {
        let output = Command::new(&self.command)
            .arg("-f")
            .arg(&self.file)
            .arg("-p")
            .arg(&self.name)
            .arg("ps")
            .arg("-aq")
            .output()?
            .stdout;

        let value = String::from_utf8(output).unwrap().trim().to_string();

        Ok(!value.is_empty())
    }

    fn running(&self) -> Result<bool> {
        let output = Command::new(&self.command)
            .arg("-f")
            .arg(&self.file)
            .arg("-p")
            .arg(&self.name)
            .arg("ps")
            .arg("-q")
            .output()?
            .stdout;

        let value = String::from_utf8(output).unwrap().trim().to_string();

        Ok(!value.is_empty())
    }

    fn cp(&self, source: String, destination: String) -> Result<bool> {
        let mut command = Command::new(&self.command);
        command
            .arg("-f")
            .arg(&self.file)
            .arg("-p")
            .arg(&self.name)
            .arg("cp")
            .arg(source)
            .arg(format!("{}:{}", &self.name, destination));

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn exec(&self, cmd: String) -> Result<bool> {
        let mut command = Command::new(&self.command);
        command
            .arg("-f")
            .arg(&self.file)
            .arg("-p")
            .arg(&self.name)
            .arg("exec")
            .arg("-u")
            .arg(&self.user)
            .arg("-w")
            .arg(&self.workspace_folder)
            .arg(&self.service)
            .arg("sh")
            .arg("-c")
            .arg(cmd);

        print_command(&command);

        Ok(command.status()?.success())
    }
}
