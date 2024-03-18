use crate::{arguments::AnimeCommands, client::Arkalis};

pub mod add;
pub mod get;
pub mod search;

pub async fn run(command: AnimeCommands, client: Arkalis) -> anyhow::Result<()> {
    match command {
        AnimeCommands::Get { id, source, output } => get::get(id, source, client, output).await,
        AnimeCommands::Add { file } => add::add(file, client).await,
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
