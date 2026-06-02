use clap::{Parser, ValueEnum};

/// Data formats accepted as input.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Format {
    Json,
    Yaml,
    Toml,
    Xml,
    Properties,
}

/// Target output languages/modes for code generation.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum TargetMode {
    #[value(alias = "rust")]
    Rust,
    #[value(alias = "typescript")]
    Typescript,
    #[value(alias = "typescript/typealias")]
    TypescriptTypeAlias,
    #[value(alias = "kotlin/jackson")]
    KotlinJackson,
    #[value(alias = "kotlin/kotlinx")]
    KotlinKotlinx,
    #[value(alias = "json_schema")]
    JsonSchema,
    #[value(alias = "shape")]
    Shape,
}

/// CLI arguments for tyg.
#[derive(Parser)]
#[command(name = "tyg")]
pub struct Cli {
    /// One or more file paths to read and merge.
    #[arg(help = "File paths to read from")]
    pub sources: Vec<String>,

    /// Remote URLs to fetch data from, comma-separated.
    #[arg(
        short,
        long,
        help = "URL(s) to fetch data from (comma-separated)",
        value_delimiter = ','
    )]
    pub url: Vec<String>,

    /// Force a specific input format instead of auto-detecting.
    #[arg(short, long, value_enum)]
    pub format: Option<Format>,

    /// Name for the root generated type.
    #[arg(short, long, default_value = "Root")]
    pub name: String,

    /// Target language or output mode.
    #[arg(short, long, value_enum, default_value = "rust")]
    pub lang: TargetMode,
}
