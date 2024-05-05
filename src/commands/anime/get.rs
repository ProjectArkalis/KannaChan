use super::InfoSource;
use crate::anilist;
use kanna_commons::{
    aoba::Aoba,
    arkalis::Arkalis,
    repos::{anime::KannaAnime, anime_infos::AnimeInfos},
};
use log::info;
use std::path::Path;
use tokio::fs;

pub async fn get(
    id: i64,
    source: InfoSource,
    output: String,
    mut arkalis: Arkalis,
    aoba: Aoba,
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
            if let Ok(anime) = AnimeInfos::from_anime_id(id as u32, &mut arkalis).await {
                let anime = anime
                    .with_seasons(&mut arkalis)
                    .await?
                    .fix_image_urls(&aoba);
                Some(anime)
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
