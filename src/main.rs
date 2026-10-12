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

    let (merged, conflicts) = merge::merge_samples(samples);

    for conflict in &conflicts {
        let path = if conflict.path.is_empty() {
            "root"
        } else {
            conflict.path.as_str()
        };

        eprintln!(
            "tyg: warning: type conflict at `{path}`: {} -> {} (keeping the last sample)",
            conflict.existing, conflict.incoming
        );
    }

    let code = codegen::generate(&cli.name, &merged, cli.lang)?;

    println!("{code}");

    Ok(())
}
