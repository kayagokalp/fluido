mod cmd;

use clap::Parser;
use cmd::Args;
use mixer_generator::concentration::Concentration;

fn main() -> anyhow::Result<()> {
    let args = Args::try_parse()?;

    let time_limit = args.time_limit;
    let target_concentration = args.target_concentration;
    let input_space = args
        .input_space
        .iter()
        .map(|input_concentration| Concentration::from_f64(*input_concentration))
        .collect();
    println!(
        "Starting to equality saturation, this will take ~{} seconds to finish.",
        time_limit
    );

    let generated_mixer_sequence =
        mixer_generator::saturate(target_concentration, time_limit, input_space)?;

    let cost = generated_mixer_sequence.cost;
    let expr = generated_mixer_sequence.best_expr;

    println!("best expr: {expr}");
    println!("cost: {cost}");
    Ok(())
}
