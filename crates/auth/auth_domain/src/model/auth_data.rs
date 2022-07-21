#[derive(PartialEq, Eq, Debug)]
pub struct AuthData {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
}
