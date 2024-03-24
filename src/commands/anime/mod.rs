use crate::{aoba::AobaService, client::Arkalis};
use clap::{Subcommand, ValueEnum};

pub mod add;
pub mod get;
pub mod search;

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

pub async fn run(command: AnimeCommands, client: Arkalis, aoba: AobaService) -> anyhow::Result<()> {
    match command {
        AnimeCommands::Get { id, source, output } => get::get(id, source, client, output).await,
        AnimeCommands::Add { file } => add::add(file, client, aoba).await,
        //deve realmente ter algum jeito melhor de fazer isso
        AnimeCommands::Search {
            title,
            synopsis,
            is_nsfw,
            genre,
            start_release_date,
            end_release_date,
        } => {
            search::search(
                title,
                synopsis,
                is_nsfw,
                genre,
                start_release_date,
                end_release_date,
                client,
            )
            .await
        }
    }
}
