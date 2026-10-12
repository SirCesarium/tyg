use crate::cli::{Cli, Format};
use crate::error::CliError;
use crate::format;
use clap::CommandFactory;
use reqwest::blocking;
use std::fs;
use std::io::{self, IsTerminal, Read};
use std::time::Duration;

/// Give up on unresponsive hosts instead of hanging forever.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// Collects all input samples from the configured sources.
///
/// Remote URLs and local file paths can be freely mixed: every URL and every
/// file contributes samples. Stdin is read only when no other source is given;
/// on a terminal with no sources at all the help is printed instead of
/// blocking on stdin.
/// Each source is parsed according to its format (auto-detected or forced).
/// Multiple samples from the same source (JSON streaming, multi-doc YAML)
/// are flattened into a single `Vec`.
pub fn collect_samples(cli: &Cli) -> Result<Vec<serde_json::Value>, CliError> {
    let mut samples = Vec::new();

    if !cli.url.is_empty() {
        let client = blocking::Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .build()?;

        for url in &cli.url {
            let res = client.get(url).send()?.text()?;

            samples.extend(format::parse_all(&res, cli.format.unwrap_or(Format::Json))?);
        }
    }

    for path in &cli.sources {
        let content = fs::read_to_string(path)?;
        let fmt = cli.format.unwrap_or_else(|| format::detect_format(path));

        samples.extend(format::parse_all(&content, fmt)?);
    }

    if cli.url.is_empty() && cli.sources.is_empty() {
        let mut stdin = io::stdin();

        // A terminal with no arguments is a user asking what this does,
        // not a data source: show the help instead of blocking on stdin.
        if stdin.is_terminal() {
            Cli::command().print_help()?;

            return Ok(samples);
        }

        let mut buffer = String::new();

        stdin.read_to_string(&mut buffer)?;

        if !buffer.trim().is_empty() {
            let fmt = cli.format.unwrap_or(Format::Json);

            samples.extend(format::parse_all(&buffer, fmt)?);
        }
    }

    Ok(samples)
}
