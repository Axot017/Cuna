use auth_domain::model::auth_data::AuthData;

use crate::entity::auth_data_entity::AuthDataEntity;

impl Into<AuthData> for AuthDataEntity {
    fn into(self) -> AuthData {
        AuthData {
            id: self.id,
            name: self.name,
            email: self.email,
            password: self.password,
        }
    }
}
