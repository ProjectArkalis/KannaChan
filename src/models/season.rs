use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Season {
    pub id: Option<u32>,
    pub name: String,
    pub cover: Option<String>
}