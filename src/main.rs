use crate::configs::Configs;
use arguments::{Cli, Commands};
use arkalis::arkalis_core_service_client::ArkalisCoreServiceClient;
use clap::Parser;
use commands::{login, set_client};

mod arguments;
mod configs;
mod models;
mod commands;

pub mod arkalis {
    tonic::include_proto!("arkalis");
}

lazy_static::lazy_static! {
    pub static ref CONFIGS: Configs = Configs::get();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let client = ArkalisCoreServiceClient::connect(CONFIGS.arkalis_url.clone()).await?;
    set_client(client).await;

    match args.command {
        Commands::Login { name, admin_key } => login::login(name, admin_key).await,
    }
}
