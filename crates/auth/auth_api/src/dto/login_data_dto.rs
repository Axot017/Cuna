use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct LoginDataDto {
    pub login: String,
    pub password: String,
}
