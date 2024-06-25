use std::str::FromStr;

use fluido_core::{search_mixer_design, Config};
use fluido_types::fluid::{Fluid, Number};

use crate::{manifest::TestManifest, util::run_and_capture_output};

pub async fn run_saturation(
    manifest: &TestManifest,
    config: Config,
) -> anyhow::Result<(bool, String)> {
    let (result, output) = run_and_capture_output(|| async {
        let setup = &manifest.setup;
        let expected = &manifest.expected;
        let input_fluids = setup
            .input
            .values()
            .map(|input_fluid| {
                let fluid_str = format!(
                    "(fluid {} {})",
                    input_fluid.concentration, input_fluid.volume
                );
                // Convert the error into anyhow error.
                Fluid::from_str(&fluid_str).map_err(|err| err.into())
            })
            .collect::<anyhow::Result<Vec<Fluid<Number>>>>()?;
        let target_fluids = setup
            .target
            .values()
            .map(|input_fluid| {
                let fluid_str = format!(
                    "(fluid {} {})",
                    input_fluid.concentration, input_fluid.volume
                );
                // Convert the error into anyhow error.
                Fluid::from_str(&fluid_str).map_err(|err| err.into())
            })
            .collect::<anyhow::Result<Vec<Fluid<Number>>>>()?;

        let target_concentration: Number = *target_fluids[0].concentration();
        let mixer_design =
            search_mixer_design::<Number>(config, target_concentration, input_fluids.as_ref())?;

        let mut result = true;
        if let Some(mixer_sequence) = &expected.mixer_sequence {
            let test_design = mixer_design.mixer_expr().to_string();
            result &= *mixer_sequence == test_design;
        }
        if let Some(storage_units) = expected.storage_units {
            let test_storage_units = mixer_design.storage_units_needed();
            result &= storage_units == test_storage_units;
        }

        anyhow::Ok(result)
    })
    .await;

    let run_result = result.unwrap_or_default();
    Ok((run_result, output))
}
