#[derive(PartialEq, Clone, Debug)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_exp: isize,
    pub refresh_token_exp: isize,
}
