pub type Result<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Clone, Debug)]
pub enum Error {
    RequestNotValid,
}
