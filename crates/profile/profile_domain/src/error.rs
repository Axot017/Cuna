use common_domain::db::DBError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Clone, Debug)]
pub enum Error {
    NameOrEmailNotUnique,
    DbConnectionError,
}

impl From<DBError> for Error {
    fn from(_: DBError) -> Self {
        Self::DbConnectionError
    }
}
