use clap::{Parser, ValueEnum};
use json_typegen_shared::{Options, OutputMode, codegen};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    Json,
    Yaml,
    Toml,
    Xml,
    Properties,
}

#[derive(Parser)]
#[command(name = "mold")]
struct Cli {
    #[arg(help = "File paths to read from")]
    sources: Vec<String>,

    #[arg(short, long, help = "URL to fetch data from")]
    url: Option<String>,

    #[arg(short, long, value_enum)]
    format: Option<Format>,

    #[arg(short, long, default_value = "Root")]
    name: String,
}

fn parse_to_json(input: &str, format: Format) -> Result<Value, Box<dyn std::error::Error>> {
    match format {
        Format::Json => Ok(serde_json::from_str(input)?),
        Format::Yaml => Ok(serde_yaml::from_str(input)?),
        Format::Toml => Ok(toml::from_str(input)?),
        Format::Xml => Ok(quick_xml::de::from_str(input)?),
        Format::Properties => {
            let props = java_properties::read(input.as_bytes())?;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut json_samples = Vec::new();

    if let Some(url) = cli.url {
        let res = reqwest::blocking::get(&url)?.text()?;
        json_samples.push(parse_to_json(&res, cli.format.unwrap_or(Format::Json))?);
    } else if !cli.sources.is_empty() {
        for path in &cli.sources {
            let res = fs::read_to_string(path)?;
            let fmt = cli.format.unwrap_or_else(|| {
                if path.ends_with(".json") {
                    Format::Json
                } else if path.ends_with(".yaml") || path.ends_with(".yml") {
                    Format::Yaml
                } else if path.ends_with(".toml") {
                    Format::Toml
                } else if path.ends_with(".xml") {
                    Format::Xml
                } else if path.ends_with(".properties") {
                    Format::Properties
                } else {
                    Format::Json
                }
            });
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
    options.output_mode = OutputMode::Rust;

    let code = codegen(&cli.name, &json_string, options)?;
    println!("{}", code);

    Ok(())
}

