use serde::Deserialize;
use validator::Validate;

#[derive(Validate, Deserialize, PartialEq, Clone, Debug)]
pub struct CreateProfileDto {
    #[validate(length(min = 3, max = 30))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 1024))]
    pub password: String,
}
