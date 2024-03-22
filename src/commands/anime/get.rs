use crate::{
    anilist,
    arguments::InfoSource,
    arkalis::{CreateAnimeRequest, GetAnimeByIdRequest},
    client::Arkalis,
    models::anime::ArkalisAnime,
};
use std::path::Path;
use tokio::fs;

pub async fn get(
    id: i64,
    source: InfoSource,
    mut client: Arkalis,
    output: String,
) -> anyhow::Result<()> {
    let anime = match source {
        InfoSource::Anilist => {
            if let Ok(media) = anilist::get_media(id).await {
                let seasons = anilist::get_season(id).await.unwrap();

                Some(ArkalisAnime {
                    arkalis_id: None,
                    anime: CreateAnimeRequest::from(media),
                    seasons,
                })
            } else {
                None
            }
        }
        InfoSource::Arkalis => {
            if let Ok(resp) = client
                .get_anime_by_id(GetAnimeByIdRequest { id: id as u32 })
                .await
            {
                if let Some(anime) = resp.into_inner().anime {
                    Some(ArkalisAnime {
                        arkalis_id: Some(anime.id),
                        anime: CreateAnimeRequest::from(anime),
                        seasons: vec![],
                    })
                } else {
                    None
                }
            } else {
                None
            }
        }
    };

    if let Some(anime) = anime {
        let mut path = Path::new(&output).to_owned();

        if path.is_dir() {
            path = path.join(format!("{}.json", id));
        }

        fs::write(&path, serde_json::to_string_pretty(&anime)?).await?;
        println!(";) Informações salvas em: {}", path.display());
    } else {
        println!(":( Anime não encontrado");
    }

    Ok(())
}
