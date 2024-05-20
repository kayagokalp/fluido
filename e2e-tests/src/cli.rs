use clap::Parser;

/// E2E Test suite for fluido.
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Only run tests matching this regex
    #[arg(value_name = "REGEX")]
    pub include: Option<regex::Regex>,

    /// Exclude tests matching this regex
    #[arg(long, short, value_name = "REGEX")]
    pub exclude: Option<regex::Regex>,

    /// Print out warnings, errors, and output of print options
    #[arg(long, env = "FLUIDO_TEST_VERBOSE")]
    pub verbose: bool,

    /// Update all output files
    #[arg(long)]
    pub update_output_files: bool,
}

#[derive(Debug, Clone)]
pub struct FilterConfig {
    pub include: Option<regex::Regex>,
    pub exclude: Option<regex::Regex>,
}

#[derive(Debug, Clone)]
pub struct RunConfig {
    pub verbose: bool,
    pub update_output_files: bool,
}
