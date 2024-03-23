use crate::{
    arkalis_api::{AnimeInAnimeList, AnimeList},
    models::anime::anime_lists::KannaAnimeList,
};

impl From<KannaAnimeList> for AnimeInAnimeList {
    fn from(value: KannaAnimeList) -> Self {
        match value {
            KannaAnimeList::Anilist(id) => AnimeInAnimeList {
                id_in_list: id.to_string(),
                anime_list: AnimeList::Anilist.into(),
            },
            KannaAnimeList::MyAnimeList(id) => AnimeInAnimeList {
                id_in_list: id.to_string(),
                anime_list: AnimeList::MyAnimeList.into(),
            },
        }
    }
}
