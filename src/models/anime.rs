use super::{genres::Genre, season::Season};
use crate::{
    anilist::get_media::GetMediaMedia,
    arkalis::{
        Anime, AnimeInAnimeList, AnimeList, CreateAnimeRequest, EditAnimeRequest,
        Title, TitleType,
    },
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArkalisAnime {
    pub arkalis_id: Option<u32>,
    pub anime: CreateAnimeRequest,
    pub seasons: Vec<Season>,
}

impl From<ArkalisAnime> for EditAnimeRequest {
    fn from(value: ArkalisAnime) -> Self {
        EditAnimeRequest {
            id: value.arkalis_id.unwrap(),
            anime_in_lists: value.anime.anime_in_lists,
            banner_id: value.anime.banner_id,
            genre: value.anime.genre,
            release_date: value.anime.release_date,
            synopsis: value.anime.synopsis,
            thumbnail_id: value.anime.thumbnail_id,
            titles: value.anime.titles,
        }
    }
}

impl From<Anime> for CreateAnimeRequest {
    fn from(value: Anime) -> Self {
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

impl From<GetMediaMedia> for CreateAnimeRequest {
    fn from(value: GetMediaMedia) -> Self {
        let title = value.title.unwrap();

        let date = value.start_date.unwrap();

        let naive_date = NaiveDate::from_ymd_opt(
            date.year.unwrap() as i32,
            date.month.unwrap() as u32,
            date.day.unwrap() as u32,
        )
        .unwrap();
        let datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();

        let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(datetime, Utc);

        CreateAnimeRequest {
            is_hidden: false,
            is_nsfw: value.is_adult.unwrap(),
            anime_in_lists: vec![
                AnimeInAnimeList {
                    id_in_list: value.id_mal.unwrap().to_string(),
                    anime_list: AnimeList::MyAnimeList.into(),
                },
                AnimeInAnimeList {
                    id_in_list: value.id.to_string(),
                    anime_list: AnimeList::Anilist.into(),
                },
            ],
            titles: vec![
                Title {
                    name: title.english.unwrap(),
                    is_main: false,
                    title_type: TitleType::English.into(),
                },
                Title {
                    name: title.romaji.unwrap(),
                    is_main: true,
                    title_type: TitleType::Romaji.into(),
                },
                Title {
                    name: title.native.unwrap(),
                    is_main: false,
                    title_type: TitleType::Native.into(),
                },
            ],
            release_date: datetime.timestamp(),
            synopsis: value.description.unwrap(),
            thumbnail_id: None,
            banner_id: None,
            genre: value
                .genres
                .unwrap()
                .into_iter()
                .map(|x| Genre::from(x.unwrap()).bits())
                .sum(),
        }
    }
}
