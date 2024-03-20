use clap::{Parser, Subcommand, ValueEnum};

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

#[derive(Debug, Clone, ValueEnum)]
pub enum InfoSource {
    Anilist,
    Arkalis,
}

#[derive(Debug, Subcommand)]
pub enum AnimeCommands {
    /// Pega informações de um anime da AniList
    Get {
        /// Id do anime
        #[arg(short, long)]
        id: i64,
        /// Fonte das informações
        #[arg(short, long)]
        #[clap(value_enum, default_value_t = InfoSource::Anilist)]
        source: InfoSource,
        /// Arquivo ou pasta onde serão salvas as informações, se for uma pasta sera salvo como {id}.json
        #[arg(short, long)]
        output: String,
    },
    /// Adiciona um anime ou atualiza um anime (id necessario)
    Add {
        /// Arquivo json com as informações do anime
        #[arg(short, long)]
        file: String,
    },
    //deve ter algum jeito melhor de fazer isso
    Search {
        #[arg(short, long)]
        title: Option<String>,
        #[arg(short, long)]
        synopsis: Option<String>,
        #[arg(short, long)]
        is_nsfw: Option<bool>,
        #[arg(short, long)]
        genre: Option<u64>,
        #[arg(short = 'r', long)]
        start_release_date: Option<i64>,
        #[arg(short, long)]
        end_release_date: Option<i64>,
    },
}
