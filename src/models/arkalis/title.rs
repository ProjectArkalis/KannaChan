use crate::{
    arkalis_api::{Title, TitleType},
    models::anime::title::{KannaTitle, KannaTitleType},
};

impl From<KannaTitle> for Title {
    fn from(value: KannaTitle) -> Self {
        Title {
            is_main: value.is_main,
            name: value.name,
            title_type: match value.title_type {
                KannaTitleType::Romaji => TitleType::Romaji.into(),
                KannaTitleType::English => TitleType::English.into(),
                KannaTitleType::Portuguese => TitleType::Portuguese.into(),
                KannaTitleType::Native => TitleType::Native.into(),
            },
        }
    }
}
