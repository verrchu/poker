use ::thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unexpected game value: {0}")]
    GameParseError(String),
    #[error("unexpected rank value: {0}")]
    RankParseError(String),
    #[error("unexpected suite value: {0}")]
    SuiteParseError(String),
}
