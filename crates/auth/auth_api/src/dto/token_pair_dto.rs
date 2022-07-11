use serde::Serialize;

#[derive(Serialize, PartialEq, Clone, Debug)]
pub struct TokenPairDto {
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_exp: isize,
    pub refresh_token_exp: isize,
}
