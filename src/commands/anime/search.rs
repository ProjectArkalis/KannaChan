use crate::models::search_table::SearchTable;
use kanna_commons::{arkalis::Arkalis, repos::anime_infos::AnimeInfos};
use tabled::Table;

pub async fn search(
    title: Option<String>,
    synopsis: Option<String>,
    is_nsfw: Option<bool>,
    genre: Option<u64>,
    start_release_date: Option<i64>,
    end_release_date: Option<i64>,
    mut arkalis: Arkalis,
) -> anyhow::Result<()> {
    let b = AnimeInfos::search(
        title,
        synopsis,
        is_nsfw,
        genre,
        start_release_date,
        end_release_date,
        &mut arkalis,
    )
    .await?
    .into_iter()
    .map(SearchTable::from)
    .collect::<Vec<SearchTable>>();

    let tabled = Table::new(b).to_string();
    println!("{tabled}");

    Ok(())
}
