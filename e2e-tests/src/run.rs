use std::{
    io::{stdout, Write},
    time::Instant,
};

use crate::{
    cli::{FilterConfig, RunConfig},
    harness,
    util::{discover_test_configs, VecExt},
};
use colored::Colorize;
use fluido_core::{Config, LogConfig, MixerGenerationConfig, MixerGenerator};

pub async fn run(run_config: &RunConfig, filter_config: &FilterConfig) -> anyhow::Result<()> {
    let mut discovered_tests = discover_test_configs()?;
    let total_test_count = discovered_tests.len();
    let included_tests = filter_config
        .include
        .as_ref()
        .map(|include| {
            discovered_tests.retained(|t| include.is_match(&t.test_manifest.metadata.name))
        })
        .unwrap_or_default();
    let excluded_tests = filter_config
        .exclude
        .as_ref()
        .map(|exclude| {
            discovered_tests.retained(|t| !exclude.is_match(&t.test_manifest.metadata.name))
        })
        .unwrap_or_default();

    let disabled_tests = discovered_tests.retained(|t| !t.test_manifest.disabled);

    let mut number_of_tests_executed = 0;
    let mut number_of_tests_failed = 0;

    let instant = Instant::now();
    for test_file in discovered_tests.iter() {
        let test_manifest = &test_file.test_manifest;

        print!("Testing {}...", test_manifest.metadata.name);
        stdout().flush().unwrap();

        let time_limit = test_manifest.time_limit;
        // TODO: expose this to the test toml.
        let mixer_generator = MixerGenerator::EqualitySaturation;
        let mixer_config = MixerGenerationConfig::new(time_limit, mixer_generator);
        // TODO: expose extra logging steps to the test toml.
        let logging = LogConfig::silent();
        let config = Config::new(mixer_config, logging);
        // Runs the search_mixer_design routine with test setup
        let (result, output) = harness::run_saturation(test_manifest, config).await?;
        if !result {
            number_of_tests_failed += 1;
            println!("{}", "FAILED".red());
        } else {
            println!("{}", "ok".green());
        }
        if run_config.verbose {
            println!("--- OUTPUT ---");
            println!("{output}");
        }
        number_of_tests_executed += 1;
    }
    let duration = instant.elapsed();

    if number_of_tests_executed == 0 {
        if let Some(include) = &filter_config.include {
            println!(
                "Filtered {} tests with `include` regex: {:?}",
                included_tests.len(),
                include
            );
        }
        if let Some(exclude) = &filter_config.exclude {
            println!(
                "Filtered {} tests with `exclude` regex: {:?}",
                excluded_tests.len(),
                exclude
            );
        }
        if !disabled_tests.is_empty() {
            println!("{} tests were disabled.", disabled_tests.len());
        }

        if total_test_count == 0 {
            println!("No test found!");
        }
    } else {
        println!("_________________________________");
        println!(
            "Fluido test results: {}. {} total, {} passed, {} failed; {} disabled [test duration: {} seconds]",
            if number_of_tests_failed == 0 {
                "ok".green().bold()
            }else {
                "failed".red().bold()
            },
            total_test_count,
            number_of_tests_executed - number_of_tests_failed,
            number_of_tests_failed,
            disabled_tests.len(),
            duration.as_secs()
        )
    }
    if number_of_tests_failed == 0 {
        Ok(())
    } else {
        anyhow::bail!("there are failing tests")
    }
}
