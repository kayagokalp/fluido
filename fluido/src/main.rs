mod cmd;

use clap::Parser;
use cmd::Args;
use fluido_core::{Config, LogConfig, MixerGenerationConfig, MixerGenerator};
use fluido_types::{concentration::Concentration, fluid::Fluid};

fn main() -> anyhow::Result<()> {
    let args = Args::try_parse()?;
    handle_args(args)?;
    Ok(())
}

fn handle_args(args: Args) -> anyhow::Result<()> {
    println!(
        "Starting to equality saturation, this will take ~{} seconds to finish.",
        args.time_limit
    );
    let target_concentration = Concentration::from(args.target_concentration);
    let input_space = args
        .input_space
        .iter()
        .map(|input_concentration| {
            let conc = Concentration::from(*input_concentration);
            //TODO: Actually parse fluid vol from user.
            Fluid::new(conc, 1.0.into())
        })
        .collect::<Vec<_>>();
    let config = Config::from(args);

    let mixer_design =
        fluido_core::search_mixer_design(config, target_concentration, &input_space)?;

    println!("best expr: {}", mixer_design.mixer_expr());
    println!("cost: {}", mixer_design.cost());
    println!(
        "need at least {} storage units.",
        mixer_design.storage_units_needed()
    );

    Ok(())
}

impl From<Args> for Config {
    fn from(value: Args) -> Self {
        let time_limit = value.time_limit;

        let mixer_generation_config =
            MixerGenerationConfig::new(time_limit, MixerGenerator::EqualitySaturation);
        let logging_config = LogConfig::new(
            value.show_dot,
            value.show_ir,
            value.show_liveness,
            value.show_interference,
        );

        Config::new(mixer_generation_config, logging_config)
    }
}
