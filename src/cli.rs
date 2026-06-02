use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Format {
    Json,
    Yaml,
    Toml,
    Xml,
    Properties,
}

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

#[derive(Parser)]
#[command(name = "tyg")]
pub struct Cli {
    #[arg(help = "File paths to read from")]
    pub sources: Vec<String>,

    #[arg(short, long, help = "URL(s) to fetch data from (comma-separated)", value_delimiter = ',')]
    pub url: Vec<String>,

    #[arg(short, long, value_enum)]
    pub format: Option<Format>,

    #[arg(short, long, default_value = "Root")]
    pub name: String,

    #[arg(short, long, value_enum, default_value = "rust")]
    pub lang: TargetMode,
}
