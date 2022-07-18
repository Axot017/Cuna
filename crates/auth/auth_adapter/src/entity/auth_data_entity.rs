#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AuthDataEntity {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub password: String,
}
