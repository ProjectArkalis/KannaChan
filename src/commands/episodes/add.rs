use kanna_commons::{
    arkalis::Arkalis,
    repos::{anime_infos::AnimeInfos, episode::KannaEpisode, source::KannaSource},
};
use tokio::fs;

pub async fn add(
    file: String,
    source: u32,
    season: usize,
    sequence: u32,
    url: String,
    mut arkalis: Arkalis,
) -> anyhow::Result<()> {
    if KannaSource::from_id(source, &mut arkalis).await.is_ok() {
        let contents = fs::read(&file).await?;
        let contents_str = String::from_utf8(contents)?;

        let mut anime = serde_json::from_str::<AnimeInfos>(&contents_str)?;

        if anime.seasons.len() > season {
            if let Some(season_id) = anime.seasons[season].id {
                let mut ep = KannaEpisode {
                    lbry_url: Some(url),
                    ..Default::default()
                };

                ep.create_episode(season_id, source, sequence, &mut arkalis)
                    .await?;

                if let Some(s) = anime.seasons[season]
                    .sources
                    .iter_mut()
                    .find(|x| x.id == Some(source))
                {
                    s.episodes.push(ep);
                } else {
                    let mut s = KannaSource::from_id(source, &mut arkalis).await?;
                    s.episodes.push(ep);
                    anime.seasons[season].sources.push(s)
                }

                fs::write(&file, serde_json::to_string_pretty(&anime)?).await?;

            } else {
                println!("Temporada sem id")
            }
        } else {
            println!("Temporada não existe")
        }
    } else {
        println!("Source não existe")
    }

    Ok(())
}
