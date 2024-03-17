use chrono::{DateTime, NaiveDate, Utc};
use crate::{anilist::get_media::GetMediaMedia, arkalis::{AnimeInAnimeList, AnimeList, CreateAnimeRequest, Title, TitleType}};


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
            genre: 1
        }
    }
}