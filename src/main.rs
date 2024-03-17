use crate::configs::Configs;
use arguments::{Cli, Commands};
use clap::Parser;
use client::get_client;
use commands::login;

mod arguments;
mod client;
mod commands;
mod configs;
mod models;

pub mod arkalis {
    tonic::include_proto!("arkalis");
}

lazy_static::lazy_static! {
    pub static ref CONFIGS: Configs = Configs::get();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let client = get_client(CONFIGS.token.clone()).await?;

    match args.command {
        Commands::Login { name, admin_key } => login::login(name, admin_key, client).await,
    }
}
