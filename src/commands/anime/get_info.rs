use tokio::fs;

use crate::{anilist, arkalis::CreateAnimeRequest, models::anime::ArkalisAnime};

pub async fn get_info(id: i64) -> anyhow::Result<()> {
    let media = anilist::get_media(id).await?;
    let anime = ArkalisAnime {
        arkalis_id: None,
        anime: CreateAnimeRequest::from(media),
    };

    fs::write(format!("{}.json", id), serde_json::to_string_pretty(&anime)?).await?;
    println!("Informações salvas ;)");

    Ok(())
}
