use serde::Deserialize;

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct CreateProfileDto {
    pub name: String,
    pub email: String,
    pub password: String,
}
