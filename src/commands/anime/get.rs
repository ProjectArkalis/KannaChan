use super::InfoSource;
use crate::{
    anilist,
    arkalis_api::{GetAnimeByIdRequest, GetAnimeSeasonsRequest},
    client::Arkalis,
    models::{
        anime::{anime_infos::AnimeInfos, KannaAnime},
        season::KannaSeason,
    },
};
use log::info;
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

                Some(AnimeInfos {
                    arkalis_id: None,
                    anime: KannaAnime::from(media),
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
                    let seasons = client
                        .get_anime_seasons(GetAnimeSeasonsRequest { anime_id: anime.id })
                        .await?
                        .into_inner()
                        .seasons;

                    Some(AnimeInfos {
                        arkalis_id: Some(anime.id),
                        anime: KannaAnime::from(anime),
                        seasons: seasons
                            .iter()
                            .map(|x| KannaSeason {
                                id: Some(x.id),
                                name: x.name.clone(),
                                thumbnail: x.cover_id.clone(),
                            })
                            .collect::<Vec<KannaSeason>>(),
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
        info!(";) Informações salvas em: {}", path.display());
    } else {
        println!(":( Anime não encontrado");
    }

    Ok(())
}
