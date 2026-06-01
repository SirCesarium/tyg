use crate::cli::TargetMode;
use crate::error::CliError;
use json_typegen_shared::{codegen, Options, OutputMode};
use serde_json::Value;

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
