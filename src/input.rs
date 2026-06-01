use crate::cli::{Cli, Format};
use crate::error::CliError;
use crate::format;
use reqwest::blocking;
use serde_json::Value;
use std::fs;
use std::io::{self, Read};

pub fn collect_samples(cli: &Cli) -> Result<Vec<Value>, CliError> {
    let mut samples = Vec::new();

    if let Some(url) = &cli.url {
        let res = blocking::get(url)?.text()?;
        samples.push(format::parse_to_json(
            &res,
            cli.format.unwrap_or(Format::Json),
        )?);
    } else if !cli.sources.is_empty() {
        for path in &cli.sources {
            let content = fs::read_to_string(path)?;
            let fmt = cli.format.unwrap_or_else(|| format::detect_format(path));
            samples.push(format::parse_to_json(&content, fmt)?);
        }
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        if !buffer.trim().is_empty() {
            let fmt = cli.format.unwrap_or(Format::Json);
            if fmt == Format::Json {
                let de = serde_json::Deserializer::from_str(&buffer);
                for value in de.into_iter::<Value>() {
                    samples.push(value?);
                }
            } else {
                samples.push(format::parse_to_json(&buffer, fmt)?);
            }
        }
    }

    Ok(samples)
}
