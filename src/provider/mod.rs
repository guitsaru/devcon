pub(crate) mod docker;
pub(crate) mod docker_compose;
pub(crate) mod podman;
pub(crate) mod podman_compose;

use colored::Colorize;
use std::io::Result;

pub(crate) trait Provider {
    fn build(&self, use_cache: bool) -> Result<bool>;
    fn create(&self) -> Result<bool>;
    fn start(&self) -> Result<bool>;
    fn stop(&self) -> Result<bool>;
    fn restart(&self) -> Result<bool>;
    fn attach(&self) -> Result<bool>;
    fn rm(&self) -> Result<bool>;
    fn exists(&self) -> Result<bool>;
    fn running(&self) -> Result<bool>;
    fn cp(&self, source: String, destination: String) -> Result<bool>;
    fn exec(&self, cmd: String) -> Result<bool>;
}

pub(crate) fn print_command(command: &std::process::Command) {
    let exec = command.get_program();
    let args: Vec<&str> = command
        .get_args()
        .map(|arg| arg.to_str().unwrap())
        .collect();

    let output = format!("{} {}", exec.to_str().unwrap(), args.join(" "));
    println!("{}", output.bold().blue());
}
