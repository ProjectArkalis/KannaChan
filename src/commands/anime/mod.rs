use crate::{arguments::AnimeCommands, client::Arkalis};

pub mod get;
pub mod add;

pub async fn run(command: AnimeCommands, client: Arkalis) -> anyhow::Result<()> {
    match command {
        AnimeCommands::Get { id, source, output } => get::get(id, source, client, output).await,
        AnimeCommands::Add { file } => add::add(file, client).await
    }
}
