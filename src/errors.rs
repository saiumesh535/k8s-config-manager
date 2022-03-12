use serde_json;
use serde_yaml;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("data store disconnected")]
    ParseError(#[from] serde_json::Error),
    #[error("fiailed to read file")]
    FileReadError(#[from] io::Error),
    #[error("fiailed to read file")]
    ToYamlError(#[from] serde_yaml::Error),
}
