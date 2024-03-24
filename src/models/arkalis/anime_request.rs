use crate::arkalis_api::CreateAnimeRequest;
use crate::arkalis_api::{self, EditAnimeRequest};
use crate::models::anime::anime_infos::AnimeInfos;
use crate::models::anime::KannaAnime;

impl From<KannaAnime> for CreateAnimeRequest {
    fn from(value: KannaAnime) -> Self {
        Self {
            titles: value.titles.into_iter().map(|x| x.into()).collect(),
            synopsis: value.synopsis,
            //todo
            thumbnail_id: value.thumbnail_id,
            // todo
            banner_id: value.banner_id,
            is_hidden: value.is_hidden,
            is_nsfw: value.is_nsfw,
            genre: value.genre,
            release_date: value.release_date,
            anime_in_lists: value.anime_lists.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<arkalis_api::Anime> for CreateAnimeRequest {
    fn from(value: arkalis_api::Anime) -> Self {
        CreateAnimeRequest {
            titles: value.titles,
            anime_in_lists: value.anime_in_lists,
            banner_id: value.banner_id,
            genre: value.genre,
            is_hidden: value.is_hidden,
            is_nsfw: value.is_nsfw,
            release_date: value.release_date,
            synopsis: value.synopsis,
            thumbnail_id: value.thumbnail_id,
        }
    }
}

impl From<AnimeInfos> for EditAnimeRequest {
    fn from(value: AnimeInfos) -> Self {
        EditAnimeRequest {
            id: value.arkalis_id.unwrap(),
            anime_in_lists: value
                .anime
                .anime_lists
                .into_iter()
                .map(|x| x.into())
                .collect(),
            genre: value.anime.genre,
            release_date: value.anime.release_date,
            synopsis: value.anime.synopsis,
            titles: value.anime.titles.into_iter().map(|x| x.into()).collect(),
            banner_id: value.anime.banner,
            thumbnail_id: value.anime.thumbnail,
        }
    }
}
