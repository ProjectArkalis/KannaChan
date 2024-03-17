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
        ///Display name
        #[arg(short, long)]
        name: String,
        ///Admin key
        #[arg(short, long)]
        admin_key: Option<String>
    }
}
