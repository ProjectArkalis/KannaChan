use crate::{arkalis::EditAnimeRequest, client::Arkalis, models::anime::ArkalisAnime};
use tokio::fs;

pub async fn add(file: String, mut client: Arkalis) -> anyhow::Result<()> {
    let contents = fs::read(&file).await?;
    let contents_str = String::from_utf8(contents)?;

    let mut anime = serde_json::from_str::<ArkalisAnime>(&contents_str)?;

    if let Some(id) = anime.arkalis_id {
        client.edit_anime(EditAnimeRequest::from(anime)).await?;
        
        println!("Atualizado anime com o id: {}", id);
    } else {
        let resp = client.create_anime(anime.anime.clone()).await?;
        let id = resp.into_inner().id;

        anime.arkalis_id = Some(id);
        fs::write(file, serde_json::to_string_pretty(&anime)?).await?;

        println!("Anime adicionado com o id: {}", id);
    }

    Ok(())
}
