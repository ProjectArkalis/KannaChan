use crate::CONFIGS;
use kanna_commons::{arkalis::Arkalis, repos::user::KannaUser};

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

    configs.save().await?;

    println!("Token salvo ;)");
    Ok(())
}
