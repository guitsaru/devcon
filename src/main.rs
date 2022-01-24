use clap::AppSettings;
use clap::Parser;
use clap::Subcommand;

pub mod commands;
pub mod devcontainers;

#[derive(Parser)]
#[clap(setting(AppSettings::ArgRequiredElseHelp))]
#[clap(setting(AppSettings::PropagateVersion))]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Start { dir: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Start { dir }) => {
            commands::start::run(dir).unwrap();
        }
        None => {}
    }
}
