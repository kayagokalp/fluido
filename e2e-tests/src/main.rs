mod cli;
mod harness;
mod manifest;
mod run;
mod util;

use clap::Parser;
use cli::{FilterConfig, RunConfig};
use run::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    let filter_config = FilterConfig {
        include: args.include,
        exclude: args.exclude,
    };
    let run_config = RunConfig {
        verbose: args.verbose,
    };

    run(&run_config, &filter_config).await?;

    Ok(())
}
