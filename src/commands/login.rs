use crate::{
    arkalis::{CreateAdminRequest, CreateTokenRequest, GetUserInfoRequest}, client::{get_client, Arkalis}, models::user::User, CONFIGS
};
use tokio::fs;

pub async fn login(
    name: String,
    admin_key: Option<String>,
    mut client: Arkalis,
) -> anyhow::Result<()> {
    let token = if let Some(key) = admin_key {
        client
            .create_admin(CreateAdminRequest {
                display_name: name,
                admin_master_key: key,
            })
            .await?
            .into_inner()
            .token
    } else {
        client
            .create_token(CreateTokenRequest { display_name: name })
            .await?
            .into_inner()
            .token
    };

    client = get_client(Some(token.clone())).await?;
    let user = client.get_user_info(GetUserInfoRequest {}).await?;
    let user = User::from(user.into_inner());

    println!("Logado como: {} [{}]; Role: {}", user.display_name, user.id, String::from(user.role));

    let mut configs = CONFIGS.clone();
    configs.token = Some(token);

    fs::write("configs.toml", toml::to_string_pretty(&configs).unwrap()).await?;

    println!("Token salvo ;)");

    Ok(())
}
