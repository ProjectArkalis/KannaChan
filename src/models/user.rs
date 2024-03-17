use crate::arkalis::GetUserInfoResponse;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum Roles {
    Admin,
    Uploader,
    User,
}

impl From<Roles> for String {
    fn from(value: Roles) -> Self {
        match value {
            Roles::Admin => "admin".to_string(),
            Roles::Uploader => "uploader".to_string(),
            Roles::User => "user".to_string(),
        }
    }
}

impl<'a> From<&'a str> for Roles {
    fn from(value: &'a str) -> Self {
        match value {
            "admin" => Roles::Admin,
            "uploader" => Roles::Uploader,
            "user" => Roles::User,
            _ => Roles::User,
        }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub display_name: String,
    pub role: Roles,
}

impl From<GetUserInfoResponse> for User {
    fn from(value: GetUserInfoResponse) -> Self {
        User {
            id: value.id,
            display_name: value.display_name,
            role: Roles::from(value.role.as_str()),
        }
    }
}
