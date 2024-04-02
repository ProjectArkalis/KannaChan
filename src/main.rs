use crate::configs::Configs;
use arguments::{Cli, Commands};
use clap::Parser;
use commands::{anime, auth, episodes, source};
use kanna_commons::get_clients;

mod anilist;
mod arguments;
mod commands;
mod configs;
mod models;

lazy_static::lazy_static! {
    pub static ref CONFIGS: Configs = Configs::get();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let (arkalis, aoba) =
        get_clients(&CONFIGS.arkalis_url, &CONFIGS.aoba_url, &CONFIGS.token).await?;

    match args.command {
        Commands::Auth { command } => auth::run(command, arkalis).await,
        Commands::Anime { command } => anime::run(command, arkalis, aoba).await,
        Commands::Source { command } => source::run(command, arkalis).await,
        Commands::Episodes { command } => episodes::run(command, arkalis).await,
    }
}
