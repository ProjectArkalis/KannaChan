use tokio::fs;

use crate::{arkalis::{CreateAdminRequest, CreateTokenRequest}, CONFIGS};

use super::get_client;

pub async fn login(name: String, admin_key: Option<String>) -> anyhow::Result<()> {
    let mut client = get_client().await;

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

    let mut configs = CONFIGS.clone();
    configs.token = Some(token);

    fs::write("configs.toml", toml::to_string_pretty(&configs).unwrap()).await?;

    println!("Token salvo ;)");

    Ok(())
}
