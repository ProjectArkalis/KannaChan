use crate::{arkalis_api::SearchAnimeRequest, client::Arkalis, models::search_table::SearchTable};
use tabled::Table;

pub async fn search(
    title: Option<String>,
    synopsis: Option<String>,
    is_nsfw: Option<bool>,
    genre: Option<u64>,
    start_release_date: Option<i64>,
    end_release_date: Option<i64>,
    mut client: Arkalis,
) -> anyhow::Result<()> {
    let a = client
        .search_anime(SearchAnimeRequest {
            title,
            synopsis,
            is_nsfw,
            genre,
            start_release_date,
            end_release_date,
        })
        .await?;

    let b = a
        .into_inner()
        .animes
        .into_iter()
        .map(SearchTable::from)
        .collect::<Vec<SearchTable>>();

    let tabled = Table::new(b).to_string();
    println!("{tabled}");

    Ok(())
}
