use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
pub struct LoginDataDto {
    pub login: String,
    pub password: String,
}
