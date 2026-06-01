use miette::Diagnostic;
use std::io;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum CliError {
    #[error("I/O error")]
    Io(#[from] io::Error),

    #[error("failed to parse {format} input: {msg}")]
    Parse { format: String, msg: String },

    #[error("network request failed")]
    Network(#[from] reqwest::Error),

    #[error("JSON error")]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    Codegen(String),
}
