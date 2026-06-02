use crate::cli::TargetMode;
use crate::error::CliError;
use json_typegen_shared::{Options, OutputMode, codegen};
use serde_json::Value;

/// Generates type-safe code from a JSON value for the given target language.
///
/// Delegates to `json_typegen_shared::codegen` under the hood.
/// The JSON is serialized to a string first, then passed to the engine
/// together with the root type name and output mode.
pub fn generate(name: &str, json: &Value, mode: TargetMode) -> Result<String, CliError> {
    let json_string = serde_json::to_string(json)?;
    let mut options = Options::default();

    options.output_mode = match mode {
        TargetMode::Rust => OutputMode::Rust,
        TargetMode::Typescript => OutputMode::Typescript,
        TargetMode::TypescriptTypeAlias => OutputMode::TypescriptTypeAlias,
        TargetMode::KotlinJackson => OutputMode::KotlinJackson,
        TargetMode::KotlinKotlinx => OutputMode::KotlinKotlinx,
        TargetMode::JsonSchema => OutputMode::JsonSchema,
        TargetMode::Shape => OutputMode::Shape,
    };

    codegen(name, &json_string, options).map_err(|e| CliError::Codegen(e.to_string()))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use serde_json::json;

    const JSON: &str = r#"{"name":"foo","count":42}"#;

    #[test]
    fn generate_rust() {
        let v: Value = serde_json::from_str(JSON).unwrap();
        let code = generate("Test", &v, TargetMode::Rust).unwrap();

        assert!(code.contains("struct Test"));
        assert!(code.contains("name: String"));
    }

    #[test]
    fn generate_typescript() {
        let v: Value = serde_json::from_str(JSON).unwrap();
        let code = generate("Test", &v, TargetMode::Typescript).unwrap();

        assert!(code.contains("interface Test"));
        assert!(code.contains("name: string"));
    }

    #[test]
    fn generate_typescript_typealias() {
        let v: Value = serde_json::from_str(JSON).unwrap();
        let code = generate("Test", &v, TargetMode::TypescriptTypeAlias).unwrap();

        assert!(code.contains("type Test"));
    }

    #[test]
    fn generate_json_schema() {
        let v: Value = serde_json::from_str(JSON).unwrap();
        let code = generate("Test", &v, TargetMode::JsonSchema).unwrap();

        assert!(code.contains("\"type\""));
        assert!(code.contains("Test"));
    }

    #[test]
    fn generate_kotlin() {
        let v: Value = serde_json::from_str(JSON).unwrap();
        let code = generate("Test", &v, TargetMode::KotlinJackson).unwrap();

        assert!(code.contains("class Test"));
    }

    #[test]
    fn generate_empty_object() {
        let v = json!({});
        let code = generate("Empty", &v, TargetMode::Rust).unwrap();

        assert!(code.contains("struct Empty"));
    }
}
