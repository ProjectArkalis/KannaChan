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
        let mut titles = vec![];

        if let Some(title) = title.english {
            titles.push(KannaTitle {
                title,
                is_main: false,
                title_type: KannaTitleTypes::English,
            });
        }

        if let Some(title) = title.romaji {
            titles.push(KannaTitle {
                title,
                is_main: true,
                title_type: KannaTitleTypes::Romaji,
            });
        }

        if let Some(title) = title.native {
            titles.push(KannaTitle {
                title,
                is_main: false,
                title_type: KannaTitleTypes::Native,
            });
        }

        KannaAnime {
            titles,
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
