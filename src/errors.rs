use ::thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unexpected game value: {0}")]
    GameParseError(String),
}
