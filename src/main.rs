use crate::configs::Configs;
use aoba::AobaService;
use arguments::{Cli, Commands};
use clap::Parser;
use client::get_client;
use commands::{anime, auth};

mod anilist;
mod aoba;
mod arguments;
mod client;
mod commands;
mod configs;
mod models;

pub mod arkalis_api {
    tonic::include_proto!("arkalis");
}

lazy_static::lazy_static! {
    pub static ref CONFIGS: Configs = Configs::get();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let client = get_client(CONFIGS.token.clone()).await?;
    let aoba = AobaService::start(&CONFIGS)?;

    match args.command {
        Commands::Auth { command } => auth::run(command, client).await,
        Commands::Anime { command } => anime::run(command, client, aoba).await,
    }
}
