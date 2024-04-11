use dirs::config_dir;
use dotenv::dotenv;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configs {
    pub arkalis_url: String,
    pub aoba_url: String,
    pub token: Option<String>,
}

impl Configs {
    pub fn get() -> Self {
        dotenv().ok();

        Figment::from(Serialized::defaults(Configs::default()))
            .merge(Env::prefixed("ARKALIS"))
            .merge(Toml::file(format!(
                "{}/configs.toml",
                get_configs_dir().unwrap()
            )))
            .extract()
            .expect("Failed to load configs")
    }
}

impl Default for Configs {
    fn default() -> Configs {
        if cfg!(debug_assertions) {
            Configs {
                arkalis_url: "http://localhost:8000".into(),
                aoba_url: "http://localhost:8001".into(),
                token: None,
            }
        } else {
            Configs {
                arkalis_url: "https://api.arkalis.org".into(),
                aoba_url: "https://aoba.arkalis.org".into(),
                token: None,
            }
        }
    }
}

impl Configs {
    pub async fn save(&self) -> anyhow::Result<()> {
        fs::write(
            format!("{}/configs.toml", get_configs_dir()?),
            toml::to_string_pretty(self).unwrap(),
        )
        .await?;
        Ok(())
    }
}

pub fn get_configs_dir() -> anyhow::Result<String> {
    let dir = format!(
        "{}/kanna-chan",
        config_dir().unwrap().to_str().unwrap().to_owned()
    );

    std::fs::create_dir_all(&dir)?;

    Ok(dir)
}
