mod cli;
mod error;

use clap::Parser;
use cli::{Cli, Format};
use error::CliError;
use json_typegen_shared::{codegen, Options, OutputMode};
use quick_xml::de;
use reqwest::blocking;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

fn parse_to_json(input: &str, format: Format) -> Result<Value, CliError> {
    match format {
        Format::Json => Ok(serde_json::from_str(input)?),
        Format::Yaml => serde_yaml::from_str(input).map_err(|e| CliError::Parse {
            format: "yaml".into(),
            msg: e.to_string(),
        }),
        Format::Toml => toml::from_str(input).map_err(|e| CliError::Parse {
            format: "toml".into(),
            msg: e.to_string(),
        }),
        Format::Xml => de::from_str(input).map_err(|e| CliError::Parse {
            format: "xml".into(),
            msg: e.to_string(),
        }),
        Format::Properties => {
            let props = java_properties::read(input.as_bytes()).map_err(|e| CliError::Parse {
                format: "properties".into(),
                msg: e.to_string(),
            })?;
            let mut map = HashMap::new();
            for (k, v) in props {
                if let Ok(b) = v.parse::<bool>() {
                    map.insert(k, json!(b));
                } else if let Ok(n) = v.parse::<i64>() {
                    map.insert(k, json!(n));
                } else {
                    map.insert(k, json!(v));
                }
            }
            Ok(serde_json::to_value(map)?)
        }
    }
}

fn detect_format(path: &str) -> Format {
    let ext = Path::new(path).extension().and_then(|e| e.to_str());
    match ext {
        Some("json") => Format::Json,
        Some("yaml") | Some("yml") => Format::Yaml,
        Some("toml") => Format::Toml,
        Some("xml") => Format::Xml,
        Some("properties") => Format::Properties,
        _ => Format::Json,
    }
}

fn run(cli: &Cli) -> Result<(), CliError> {
    let mut json_samples = Vec::new();

    if let Some(url) = &cli.url {
        let res = blocking::get(url)?.text()?;
        json_samples.push(parse_to_json(&res, cli.format.unwrap_or(Format::Json))?);
    } else if !cli.sources.is_empty() {
        for path in &cli.sources {
            let res = fs::read_to_string(path)?;
            let fmt = cli.format.unwrap_or_else(|| detect_format(path));
            json_samples.push(parse_to_json(&res, fmt)?);
        }
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        if !buffer.trim().is_empty() {
            let fmt = cli.format.unwrap_or(Format::Json);
            if fmt == Format::Json {
                let de = serde_json::Deserializer::from_str(&buffer);
                for value in de.into_iter::<Value>() {
                    json_samples.push(value?);
                }
            } else {
                json_samples.push(parse_to_json(&buffer, fmt)?);
            }
        }
    }

    if json_samples.is_empty() {
        return Ok(());
    }

    let final_json = if json_samples.len() > 1 {
        Value::Array(json_samples)
    } else {
        json_samples.remove(0)
    };

    let json_string = serde_json::to_string(&final_json)?;
    let mut options = Options::default();
    options.output_mode = match cli.lang {
        cli::TargetMode::Rust => OutputMode::Rust,
        cli::TargetMode::Typescript => OutputMode::Typescript,
        cli::TargetMode::TypescriptTypeAlias => OutputMode::TypescriptTypeAlias,
        cli::TargetMode::KotlinJackson => OutputMode::KotlinJackson,
        cli::TargetMode::KotlinKotlinx => OutputMode::KotlinKotlinx,
        cli::TargetMode::JsonSchema => OutputMode::JsonSchema,
        cli::TargetMode::Shape => OutputMode::Shape,
    };

    let code =
        codegen(&cli.name, &json_string, options).map_err(|e| CliError::Codegen(e.to_string()))?;
    println!("{code}");

    Ok(())
}

fn main() -> miette::Result<()> {
    let cli: Cli = cli::Cli::parse();
    run(&cli).map_err(miette::Report::from)?;
    Ok(())
}
