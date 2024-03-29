use super::get_media::GetMediaMedia;
use chrono::{DateTime, NaiveDate, Utc};
use kanna_commons::repos::{
    anime::{
        lists::KannaLists,
        title::{KannaTitle, KannaTitleTypes},
        KannaAnime,
    },
    genres::Genre,
};

impl From<GetMediaMedia> for KannaAnime {
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

        KannaAnime {
            titles: vec![
                KannaTitle {
                    title: title.english.unwrap(),
                    is_main: false,
                    title_type: KannaTitleTypes::English,
                },
                KannaTitle {
                    title: title.romaji.unwrap(),
                    is_main: true,
                    title_type: KannaTitleTypes::Romaji,
                },
                KannaTitle {
                    title: title.native.unwrap(),
                    is_main: false,
                    title_type: KannaTitleTypes::Native,
                },
            ],
            synopsis: value.description.unwrap(),
            thumbnail: value.cover_image.unwrap().extra_large,
            banner: value.banner_image,
            is_hidden: false,
            is_nsfw: value.is_adult.unwrap(),
            genre: value
                .genres
                .unwrap()
                .into_iter()
                .map(|x| Genre::from(x.unwrap()).bits())
                .sum(),
            release_date: datetime.timestamp(),
            anime_lists: vec![
                KannaLists::Anilist(value.id),
                KannaLists::MyAnimeList(value.id_mal.unwrap()),
            ],
        }
    }
}
