mod cli;
mod codegen;
mod error;
mod format;
mod input;
mod merge;

use clap::Parser;

fn main() -> miette::Result<()> {
    let cli = cli::Cli::parse();
    let samples = input::collect_samples(&cli)?;

    if samples.is_empty() {
        return Ok(());
    }

    let merged = merge::merge_samples(samples);
    let code = codegen::generate(&cli.name, &merged, cli.lang)?;

    println!("{code}");

    Ok(())
}
