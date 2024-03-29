use kanna_commons::repos::anime_infos::AnimeInfos;
use tabled::Tabled;

#[derive(Debug, Tabled)]
pub struct SearchTable {
    pub id: u32,
    pub title: String,
    pub genre: u64,
}

impl From<AnimeInfos> for SearchTable {
    fn from(value: AnimeInfos) -> Self {
        SearchTable {
            id: value.arkalis_id.unwrap(),
            title: value
                .anime
                .titles
                .iter()
                .find(|x| x.is_main)
                .unwrap()
                .title
                .clone(),
            genre: value.anime.genre,
        }
    }
}
