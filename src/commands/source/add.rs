use kanna_commons::{
    arkalis::Arkalis,
    repos::{source::KannaSource, source_types::SourceType},
};

pub async fn add(
    name: String,
    priority: u32,
    types: Vec<SourceType>,
    mut arkalis: Arkalis,
) -> anyhow::Result<()> {
    let mut source = KannaSource {
        id: None,
        name,
        priority,
        source_types: types,
        episodes: vec![],
    };

    source.save(&mut arkalis).await?;

    println!("Source criada com o id: {}", source.id.unwrap());
    Ok(())
}
