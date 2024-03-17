use crate::{arkalis::CreateAnimeRequest, client::Arkalis};
use tokio::fs;

pub async fn add(file: String, mut client: Arkalis) -> anyhow::Result<()> {
    let contents = fs::read(file).await?;
    let contents_str = String::from_utf8(contents)?;

    let anime = serde_json::from_str::<CreateAnimeRequest>(&contents_str)?;

    let resp = client.create_anime(anime).await?;
    println!("Anime adicionado com o id: {}", resp.into_inner().id);

    Ok(())
}
