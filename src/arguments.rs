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
        admin_key: Option<String>,
    },
    /// Gerenciar animes
    Anime {
        #[clap(subcommand)]
        command: AnimeCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum AnimeCommands {
    /// Pega informações de um anime da AniList
    Get {
        /// Id do anime
        #[arg(short, long)]
        id: i64,
    },
    /// Adiciona um anime
    Add {
        /// Arquivo json com as informações do anime
        #[arg(short, long)]
        file: String
    }
}
