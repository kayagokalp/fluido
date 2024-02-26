mod error;
use fluido_parse::parser::Parse;
use fluido_types::{concentration::Concentration, error::MixerGenerationError, expr::Expr};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Config {
    generation: MixerGenerationConfig,
}

/// Different types of mixer generation handlers.
///
/// Currently fluido only supports equality saturation for mixer generation but it will eventually
/// add support for heuristics to generate initial mixer.
#[derive(Debug, Clone)]
pub enum MixerGenerator {
    EquailtySaturation,
}

#[derive(Debug, Clone)]
pub struct MixerGenerationConfig {
    time_limit: Duration,
    generator: MixerGenerator,
}

/// Generate a mixer for the target_concentration from input space.
fn generate_mixer(
    target_concentration: Concentration,
    input_space: &[Concentration],
    mixer_generation_config: MixerGenerationConfig,
) -> Result<Expr, MixerGenerationError> {
    let mixer_generator = mixer_generation_config.generator;
    let time_limit = mixer_generation_config.time_limit.as_secs();
    match mixer_generator {
        MixerGenerator::EquailtySaturation => {
            let generated_mixer_sequence =
                mixer_generator::saturate(target_concentration, time_limit, input_space).unwrap();
            let best_expr = generated_mixer_sequence.best_expr;
            Ok(Expr::parse(&format!("{best_expr}")).unwrap())
        }
    }
}
