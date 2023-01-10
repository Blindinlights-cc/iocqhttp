use thiserror::Error;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use std::io::Error as IoError;


#[derive(Error, Debug)]
pub enum Error {
    #[error("Reqwest Error: {0}")]
    Reqwest(#[from] ReqwestError),
    #[error("Serde Error: {0}")]
    Serde(#[from] SerdeError),
    #[error("IO Error: {0}")]
    Io(#[from] IoError),
    #[error("Onebot API Error: {0}")]
    OnebotApi(String),
    #[error("Event to reply is not correct")]
    ReplyEvent,
}