use dotenv::dotenv;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configs {
    pub arkalis_url: String,
    pub token: Option<String>,
}

impl Configs {
    pub fn get() -> Self {
        dotenv().ok();
        Figment::from(Serialized::defaults(Configs::default()))
            .merge(Env::prefixed("ARKALIS"))
            .merge(Toml::file("configs.toml"))
            .extract()
            .expect("Failed to load configs")
    }
}

impl Default for Configs {
    fn default() -> Configs {
        Configs {
            arkalis_url: "http://localhost:8000".into(),
            token: None,
        }
    }
}
