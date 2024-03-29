use kanna_commons::{arkalis::Arkalis, repos::user::KannaUser};
use tokio::fs;

use crate::CONFIGS;

pub async fn recover(key: String, mut arkalis: Arkalis) -> anyhow::Result<()> {
    let user = KannaUser::from_recovery_key(key, &mut arkalis).await?;

    println!(
        "Recuperado usuario: {} [{}]; Role: {}",
        user.name,
        user.id,
        String::from(user.role)
    );

    let mut configs = CONFIGS.clone();
    configs.token = user.token;

    fs::write("configs.toml", toml::to_string_pretty(&configs).unwrap()).await?;

    println!("Token salvo ;)");
    Ok(())
}
