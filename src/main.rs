use crate::{arguments::Commands, configs::Configs};
use arguments::Cli;
use clap::Parser;
use models::user;
use reqwest::Client;
use scanln::scanln;
use tokio::fs;

mod arguments;
mod configs;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let mut configs = Configs::get();

    match args.command {
        Commands::Login {} => {
            if configs.auth.is_none() {
                let discord_url = Client::new()
                    .get(format!("{}/auth/discord", configs.arkalis_url))
                    .send()
                    .await?
                    .text()
                    .await?;
                open::that(discord_url)?;
                let code = scanln!("Codigo: ");
                let auth = Client::new()
                    .get(format!("{}/auth/grant?code={}", configs.arkalis_url, code))
                    .send()
                    .await?
                    .json::<user::Auth>()
                    .await?;

                configs.auth = Some(auth);

                // fs::write("configs.json", serde_json::to_string_pretty(&configs).unwrap()).await?;
                fs::write("configs.toml", toml::to_string_pretty(&configs).unwrap()).await?;
            } else {
                println!("Already logged in")
            }
        }
    }

    Ok(())
}
