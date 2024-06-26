use crate::CONFIGS;
use kanna_commons::{arkalis::Arkalis, repos::user::KannaUser};

pub async fn login(
    name: String,
    admin_key: Option<String>,
    mut arkalis: Arkalis,
) -> anyhow::Result<()> {
    let user = KannaUser::create_user(name, admin_key, &mut arkalis).await?;

    println!(
        "Logado como: {} [{}]; Role: {}",
        user.name,
        user.id,
        String::from(user.role)
    );

    println!(
        "\nESSE É SEU TOKEN DE RECUPERAÇÃO NÃO PERCA-O: \n{}\n",
        user.get_recovery_key(&mut arkalis).await?
    );

    let mut configs = CONFIGS.clone();
    configs.token = user.token;

    configs.save().await?;

    println!("Token salvo ;)");
    Ok(())
}
