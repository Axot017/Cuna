use crate::entity::auth_data_entity::AuthDataEntity;
use auth_domain::error::Result;
use auth_domain::model::auth_data::AuthData;
use sqlx::PgExecutor;

pub async fn get_auth_data(executor: impl PgExecutor<'_>, login: &str) -> Result<AuthData> {
    let result = sqlx::query_as!(
        AuthDataEntity,
        "
        SELECT id, email, name, password
        FROM profile 
        WHERE email = $1 OR email = $1
        ",
        login
    )
    .fetch_one(executor)
    .await;

    todo!()
}
