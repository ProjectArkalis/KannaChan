use self::get_media::GetMediaMedia;
use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use reqwest::Client;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/anilist/schema.graphql",
    query_path = "src/anilist/get_media.graphql",
    response_derives = "Debug"
)]
pub struct GetMedia;

pub async fn get_media(id: i64) -> anyhow::Result<GetMediaMedia, ()> {
    let client = Client::new();

    let vars = get_media::Variables { id };
    let resp = post_graphql::<GetMedia, _>(&client, "https://graphql.anilist.co", vars)
        .await
        .unwrap();

    if let Some(data) = resp.data {
        if let Some(media) = data.media {
            Ok(media)
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}
