use self::get_media::{GetMediaMedia, MediaRelation, MediaStatus};
use async_recursion::async_recursion;
use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use kanna_commons::repos::season::KannaSeason;
use reqwest::Client;

pub mod anime;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/anilist/schema.graphql",
    query_path = "src/anilist/get_media.graphql",
    response_derives = "Debug, PartialEq"
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

#[async_recursion]
pub async fn get_season(id: i64) -> anyhow::Result<Vec<KannaSeason>, ()> {
    let mut seasons = vec![];

    let media = get_media(id).await?;
    seasons.push(KannaSeason {
        id: None,
        name: media
            .title
            .as_ref()
            .unwrap()
            .romaji
            .as_ref()
            .unwrap()
            .clone(),
        thumbnail: media.cover_image.as_ref().unwrap().extra_large.clone(),
    });

    if let Some(relations) = media.relations {
        let edges = relations.edges.unwrap();
        let nodes = relations.nodes.unwrap();

        let sequel_i = edges.into_iter().position(|x| {
            x.as_ref().unwrap().relation_type.as_ref().unwrap() == &MediaRelation::SEQUEL
        });

        if let Some(sequel_i) = sequel_i {
            if let Some(sequel) = &nodes[sequel_i] {
                let status = sequel.status.as_ref().unwrap();
                if status == &MediaStatus::FINISHED || status == &MediaStatus::RELEASING {
                    seasons.append(&mut get_season(sequel.id).await?)
                }
            }
        }
    }

    Ok(seasons)
}
