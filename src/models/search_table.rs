use crate::arkalis_api::Anime;
use tabled::Tabled;

#[derive(Debug, Tabled)]
pub struct SearchTable {
    pub id: u32,
    pub title: String,
    pub genre: u64,
}

impl From<Anime> for SearchTable {
    fn from(value: Anime) -> Self {
        SearchTable {
            id: value.id,
            title: value
                .titles
                .iter()
                .find(|x| x.is_main)
                .unwrap()
                .name
                .clone(),
            genre: value.genre,
        }
    }
}
