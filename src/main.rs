mod cli;
mod codegen;
mod error;
mod format;
mod input;

use clap::Parser;
use serde_json::Value;

fn main() -> miette::Result<()> {
    let cli = cli::Cli::parse();

    let mut samples = input::collect_samples(&cli)?;
    if samples.is_empty() {
        return Ok(());
    }

    let merged = if samples.len() > 1 {
        Value::Array(samples)
    } else {
        samples.remove(0)
    };

    let code = codegen::generate(&cli.name, &merged, cli.lang)?;
    println!("{code}");

    Ok(())
}
