use crate::{
    aoba::AobaService,
    arkalis_api::{AddSeasonRequest, CreateAnimeRequest, EditAnimeRequest, EditSeasonRequest},
    client::Arkalis,
    models::anime::anime_infos::AnimeInfos,
};
use tokio::fs;

pub async fn add(file: String, mut client: Arkalis, aoba: AobaService) -> anyhow::Result<()> {
    let contents = fs::read(&file).await?;
    let contents_str = String::from_utf8(contents)?;

    let mut anime = serde_json::from_str::<AnimeInfos>(&contents_str)?;

    if let Some(id) = anime.arkalis_id {
        anime.anime.save_images(&aoba).await?;

        client
            .edit_anime(EditAnimeRequest::from(anime.clone()))
            .await?;

        for season in anime.seasons.iter_mut().enumerate() {
            if let Some(s_id) = season.1.id {
                season.1.save_thumb(&aoba).await?;

                client
                    .edit_season(EditSeasonRequest {
                        id: s_id,
                        cover_id: season.1.thumbnail_id.clone(),
                        name: season.1.name.clone(),
                        sequence: season.0 as u32,
                    })
                    .await?;
            }
        }

        println!("Atualizado anime com o id: {}", id);
    } else {
        anime.anime.save_images(&aoba).await?;

        let resp = client
            .create_anime(CreateAnimeRequest::from(anime.anime.clone()))
            .await?;
        let id = resp.into_inner().id;

        for season in anime.seasons.iter_mut().enumerate() {
            season.1.save_thumb(&aoba).await?;

            let season_id = client
                .add_season(AddSeasonRequest {
                    name: season.1.name.clone(),
                    cover_id: season.1.thumbnail_id.clone(),
                    sequence: season.0 as u32,
                    anime_id: id,
                })
                .await?
                .into_inner()
                .id;
            season.1.id = Some(season_id)
        }

        anime.arkalis_id = Some(id);

        println!("Anime adicionado com o id: {}", id);
    }
    
    fs::write(file, serde_json::to_string_pretty(&anime)?).await?;
    Ok(())
}
