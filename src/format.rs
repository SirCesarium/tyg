use crate::cli::Format;
use crate::error::CliError;
use quick_xml::de;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::Path;

pub fn parse_to_json(input: &str, format: Format) -> Result<Value, CliError> {
    match format {
        Format::Json => Ok(serde_json::from_str(input)?),
        Format::Yaml => serde_yaml::from_str(input).map_err(|e| CliError::Parse {
            format: "yaml",
            msg: e.to_string(),
        }),
        Format::Toml => toml::from_str(input).map_err(|e| CliError::Parse {
            format: "toml",
            msg: e.to_string(),
        }),
        Format::Xml => de::from_str(input).map_err(|e| CliError::Parse {
            format: "xml",
            msg: e.to_string(),
        }),
        Format::Properties => {
            let props = java_properties::read(input.as_bytes()).map_err(|e| CliError::Parse {
                format: "properties",
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

pub fn detect_format(path: &str) -> Format {
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
