use clap::Parser;

/// Searching a mixer configuration from given input space and target concantration.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Target concentration
    #[arg(long)]
    pub target_concentration: f64,

    /// Input space, intial concentrations at hand.
    /// example_input: `--input-space 0 --input-space 0.4`
    #[arg(long)]
    pub input_space: Vec<f64>,

    /// Time limit in seconds.
    #[arg(long)]
    pub time_limit: u64,

    /// Show dot output of the produced mixer graph
    #[arg(long)]
    pub show_dot: bool,

    /// Show flat ir output of the produced mixer.
    #[arg(long)]
    pub show_ir: bool,

    /// Show liveness analysis over the flat-ir produced.
    #[arg(long)]
    pub show_liveness: bool,

    /// Show interference graph for the produced flat-ir.
    #[arg(long)]
    pub show_interference: bool,
}
