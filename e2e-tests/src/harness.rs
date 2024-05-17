use std::str::FromStr;

use fluido_types::fluid::Fluid;

use crate::{manifest::TestManifestFile, util::run_and_capture_output};

pub async fn run_saturation(test_manifest_file: &TestManifestFile) -> anyhow::Result<()> {
    let manifest = &test_manifest_file.test_manifest;
    let path = &test_manifest_file.path;
    let result = run_and_capture_output(|| async {
        let setup = &manifest.setup;
        let input_fluids = setup
            .input_fluids
            .values()
            .map(|input_fluid| {
                let fluid_str = format!(
                    "(fluid {} {})",
                    input_fluid.concentration, input_fluid.volume
                );
                // Convert the error into anyhow error.
                Fluid::from_str(&fluid_str).map_err(|err| err.into())
            })
            .collect::<anyhow::Result<Vec<Fluid>>>()
            .unwrap();
    })
    .await;
    Ok(())
}
