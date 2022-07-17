#[derive(PartialEq, Eq, Debug)]
pub struct AuthData {
    pub id: i128,
    pub name: String,
    pub email: String,
    pub password: String,
}
