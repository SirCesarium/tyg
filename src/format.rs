use crate::cli::Format;
use crate::error::CliError;
use quick_xml::de;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::path::Path;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn detect_json() {
        assert_eq!(detect_format("data.json"), Format::Json);
        assert_eq!(detect_format("a/b/c.json"), Format::Json);
    }

    #[test]
    fn detect_yaml() {
        assert_eq!(detect_format("config.yaml"), Format::Yaml);
        assert_eq!(detect_format("config.yml"), Format::Yaml);
    }

    #[test]
    fn detect_toml() {
        assert_eq!(detect_format("config.toml"), Format::Toml);
    }

    #[test]
    fn detect_xml() {
        assert_eq!(detect_format("data.xml"), Format::Xml);
    }

    #[test]
    fn detect_properties() {
        assert_eq!(detect_format("config.properties"), Format::Properties);
    }

    #[test]
    fn detect_unknown_defaults_to_json() {
        assert_eq!(detect_format("data.unknown"), Format::Json);
    }

    #[test]
    fn detect_no_extension_defaults_to_json() {
        assert_eq!(detect_format("data"), Format::Json);
    }

    #[test]
    fn parse_json_ok() {
        let v = parse_to_json(r#"{"a":1,"b":"x"}"#, Format::Json).unwrap();
        assert_eq!(v, json!({"a": 1, "b": "x"}));
    }

    #[test]
    fn parse_json_err() {
        let result = parse_to_json(r#"{invalid}"#, Format::Json);
        assert!(result.is_err());
    }

    #[test]
    fn parse_yaml_ok() {
        let v = parse_to_json("a: 1\nb: x\n", Format::Yaml).unwrap();
        assert_eq!(v, json!({"a": 1, "b": "x"}));
    }

    #[test]
    fn parse_toml_ok() {
        let v = parse_to_json("a = 1\nb = \"x\"\n", Format::Toml).unwrap();
        assert_eq!(v, json!({"a": 1, "b": "x"}));
    }

    #[test]
    fn parse_xml_ok() {
        let v = parse_to_json("<root><a>1</a><b>x</b></root>", Format::Xml).unwrap();
        assert_eq!(v, json!({"a": {"$text": "1"}, "b": {"$text": "x"}}));
    }

    #[test]
    fn parse_properties_ok() {
        let v = parse_to_json("a=1\nb=true\nc=hello\n", Format::Properties).unwrap();
        assert_eq!(v, json!({"a": 1, "b": true, "c": "hello"}));
    }

    #[test]
    fn parse_xml_err() {
        let result = parse_to_json("<bad", Format::Xml);
        assert!(result.is_err());
    }
}

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
