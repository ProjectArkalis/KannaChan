use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Image {
    pub url: String,
}
