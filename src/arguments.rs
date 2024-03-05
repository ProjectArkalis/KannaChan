use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}   

#[derive(Debug, Subcommand)]
pub enum Commands {
    ///Faz o login
    Login {
        // ///Itegration token
        // #[arg(short, long)]
        // token: String,
        // ///ID do usuario para autenticar
        // #[arg(short, long)]
        // discord_id: u64
    }
}
