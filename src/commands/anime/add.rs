use kanna_commons::{aoba::Aoba, arkalis::Arkalis, repos::anime_infos::AnimeInfos};
use tokio::fs;

pub async fn add(file: String, aoba: Aoba, mut arkalis: Arkalis) -> anyhow::Result<()> {
    let contents = fs::read(&file).await?;
    let contents_str = String::from_utf8(contents)?;

    let mut anime = serde_json::from_str::<AnimeInfos>(&contents_str)?;

    anime.save(&mut arkalis, &aoba).await?;

    fs::write(file, serde_json::to_string_pretty(&anime)?).await?;
    Ok(())
}
