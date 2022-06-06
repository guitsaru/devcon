use clap::AppSettings;
use clap::Parser;
use clap::Subcommand;

pub(crate) mod commands;
pub(crate) mod devcontainers;
pub(crate) mod docker;
pub(crate) mod docker_compose;
pub(crate) mod settings;

#[derive(Parser)]
#[clap(setting(AppSettings::PropagateVersion))]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Rebuild {
        dir: Option<String>,
        #[clap(short, long)]
        no_cache: bool,
    },
    Start {
        dir: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Start { dir }) => {
            commands::start::run(dir).unwrap();
        }
        Some(Commands::Rebuild { dir, no_cache }) => {
            commands::rebuild::run(dir, !no_cache).unwrap();
        }
        None => {
            commands::start::run(&None).unwrap();
        }
    }
}
