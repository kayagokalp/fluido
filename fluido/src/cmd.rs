use clap::Parser;

/// Searching a mixer configuration from given input space and target concantration.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Target concentration
    #[arg(short, long)]
    pub target_concentration: f64,

    /// Input space, intial concentrations at hand.
    /// example_input: [`0.2 0.3 0.4`]
    #[arg(short, long)]
    pub input_space: Vec<f64>,

    /// Time limit in seconds.
    #[arg(short, long)]
    pub time_limit: u64,
}
