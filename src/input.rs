use crate::cli::{Cli, Format};
use crate::error::CliError;
use crate::format;
use reqwest::blocking;
use std::fs;
use std::io::{self, Read};

pub fn collect_samples(cli: &Cli) -> Result<Vec<serde_json::Value>, CliError> {
    let mut samples = Vec::new();

    if !cli.url.is_empty() {
        for url in &cli.url {
            let res = blocking::get(url)?.text()?;
            samples.extend(format::parse_all(&res, cli.format.unwrap_or(Format::Json))?);
        }
    } else if !cli.sources.is_empty() {
        for path in &cli.sources {
            let content = fs::read_to_string(path)?;
            let fmt = cli.format.unwrap_or_else(|| format::detect_format(path));
            samples.extend(format::parse_all(&content, fmt)?);
        }
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        if !buffer.trim().is_empty() {
            let fmt = cli.format.unwrap_or(Format::Json);
            samples.extend(format::parse_all(&buffer, fmt)?);
        }
    }

    Ok(samples)
}
