use tokio::fs;

use crate::{anilist, arkalis::{CreateAnimeRequest}};

pub async fn get_info(id: i64) -> anyhow::Result<()> {
    let media = anilist::get_media(id).await?;

    fs::write(
        format!("{}.json", id),
        serde_json::to_string_pretty(&CreateAnimeRequest::from(media)).unwrap(),
    )
    .await?;
    println!("Informações salvas ;)");

    Ok(())
}
