use crate::{arguments::AnimeCommands, client::Arkalis};

pub mod get_info;
pub mod add;

pub async fn run(command: AnimeCommands, client: Arkalis) -> anyhow::Result<()> {
    match command {
        AnimeCommands::Get { id } => get_info::get_info(id).await,
        AnimeCommands::Add { file } => add::add(file, client).await
    }
}
