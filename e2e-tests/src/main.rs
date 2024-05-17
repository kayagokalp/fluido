mod cli;
mod harness;
mod manifest;
mod run;
mod util;

use clap::Parser;
use cli::{FilterConfig, RunConfig};
use run::run;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    let filter_config = FilterConfig {
        include: args.include,
        exclude: args.exclude,
    };
    let run_config = RunConfig {
        verbose: args.verbose,
        update_output_files: args.update_output_files,
    };

    run(&run_config, &filter_config)?;

    Ok(())
}
