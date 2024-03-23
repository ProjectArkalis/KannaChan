use crate::commands::{anime::AnimeCommands, auth::AuthCommands};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    ///Autenticar
    Auth {
        #[clap(subcommand)]
        command: AuthCommands,
    },
    /// Gerenciar animes
    Anime {
        #[clap(subcommand)]
        command: AnimeCommands,
    },
}
