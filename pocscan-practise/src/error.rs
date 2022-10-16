use anyhow;
use thiserror::Error;
//设置错误Error枚举
#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Reqwest: {0}")]
    Reqwest(String),
    #[error("tokio join error: {0}")]
    TokioJoinError(String),
    #[error("anyhow: {0}")]
    Anyhow(String),
}
// 将reqwest::Error 转换成Error类型
impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err.to_string())
    }
}

//将tokio::JoinError
impl std::convert::From<tokio::task::JoinError> for Error{
    fn from(err: tokio::task::JoinError) -> Self {
        Error::TokioJoinError(err.to_string())
    }
}

impl std::convert::From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Anyhow(err.to_string())
    }
}