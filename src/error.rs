use miette::Diagnostic;
use std::io;
use thiserror::Error;

/// Errors that can occur during the tyg pipeline.
#[derive(Debug, Error, Diagnostic)]
pub enum CliError {
    /// Filesystem or stdin read failure.
    #[error("I/O error")]
    Io(#[from] io::Error),

    /// Input could not be parsed as the expected format.
    #[error("failed to parse {format} input: {msg}")]
    Parse { format: &'static str, msg: String },

    /// HTTP request to a remote URL failed.
    #[error("network request failed")]
    Network(#[from] reqwest::Error),

    /// JSON serialization/deserialization error.
    #[error("JSON error")]
    Json(#[from] serde_json::Error),

    /// Code generation failed (e.g. unsupported shape).
    #[error("{0}")]
    Codegen(String),
}
