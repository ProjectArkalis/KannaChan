use crate::{
    arkalis::{AddSeasonRequest, EditAnimeRequest, EditSeasonRequest},
    client::Arkalis,
    models::anime::ArkalisAnime,
};
use tokio::fs;

pub async fn add(file: String, mut client: Arkalis) -> anyhow::Result<()> {
    let contents = fs::read(&file).await?;
    let contents_str = String::from_utf8(contents)?;

    let mut anime = serde_json::from_str::<ArkalisAnime>(&contents_str)?;

    if let Some(id) = anime.arkalis_id {
        client
            .edit_anime(EditAnimeRequest::from(anime.clone()))
            .await?;

        for season in anime.seasons.iter().enumerate() {
            if let Some(s_id) = season.1.id {
                client
                    .edit_season(EditSeasonRequest {
                        id: s_id,
                        cover_id: season.1.cover.clone(),
                        name: season.1.name.clone(),
                        sequence: season.0 as u32,
                    })
                    .await?;
            }
        }

        println!("Atualizado anime com o id: {}", id);
    } else {
        let resp = client.create_anime(anime.anime.clone()).await?;
        let id = resp.into_inner().id;

        for season in anime.seasons.iter_mut().enumerate() {
            let season_id = client
                .add_season(AddSeasonRequest {
                    name: season.1.name.clone(),
                    cover_id: season.1.cover.clone(),
                    sequence: season.0 as u32,
                    anime_id: id,
                })
                .await?
                .into_inner()
                .id;
            season.1.id = Some(season_id)
        }

        anime.arkalis_id = Some(id);
        fs::write(file, serde_json::to_string_pretty(&anime)?).await?;

        println!("Anime adicionado com o id: {}", id);
    }

    Ok(())
}
