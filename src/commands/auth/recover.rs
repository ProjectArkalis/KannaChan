use crate::{
    arkalis_api::{GetUserInfoRequest, RecoveryUserRequest},
    client::{get_client, Arkalis},
    models::user::User,
    CONFIGS,
};
use tokio::fs;

pub async fn recover(key: String, mut client: Arkalis) -> anyhow::Result<()> {
    let token = client
        .recovery_user(RecoveryUserRequest { recovery_key: key })
        .await?
        .into_inner()
        .token;

    client = get_client(Some(token.clone())).await?;
    let user = client.get_user_info(GetUserInfoRequest {}).await?;
    let user = User::from(user.into_inner());

    println!(
        "Recuperado usuario: {} [{}]; Role: {}",
        user.display_name,
        user.id,
        String::from(user.role)
    );

    let mut configs = CONFIGS.clone();
    configs.token = Some(token);

    fs::write("configs.toml", toml::to_string_pretty(&configs).unwrap()).await?;

    println!("Token salvo ;)");
    Ok(())
}
