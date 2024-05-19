//! Defines test.toml for fluido e2e-tests.
//! test.toml describes how an e2e test should be conducted, some fields are:
//! - `[metadata]` - this table contains metadata information, information that does not effect the result of the test but gives us information about the test.
//!   - `name`: Name of the test
//! - `[setup]` -- this table contains state of the environment before the test begins.
//!   - `[input-fluids]` -- set of fluids that are present at the beginning of the test.
//!     - 'fluid-name = { concentration = "", unit_volume = "" }'* - each fluid can be described as a `value`.
//!   `saturation-time` -- saturation time limit is for ending the saturation at specific time limit.``
//!   `saturation-node-count` -- saturation node count is the limit for ending the saturation at specific node count.
//!   `saturation-iter-limit` -- saturation iter limit is the limit for ending the saturation at specific iteration count.
//! - `[output]` -- set of fluids that we expect to find in the output.
//!   - 'fluid-name = { concentration = "", unit_volume = "" }'* - each fluid can be described as a `value`.
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

/// Root level struct for describing the `test.toml`
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TestManifest {
    pub metadata: Metadata,
    pub setup: Setup,
    #[serde(default)]
    pub disabled: bool,
    /// Time limit in seconds.
    pub time_limit: u64,
    pub expected: Expected,
}

/// Describes the metadata table of the manifest file.
/// This is the set of fields that cannot change the result of the test but offer insights for the maintainer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Metadata {
    pub name: String,
}

/// Describes the setup table of the manifest file.
/// This is the set of fields that prepares the saturation environment for the descibred test. It is basically the setup stage of the test to begin execution.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Setup {
    pub input: BTreeMap<String, TestFluid>,
    pub target: BTreeMap<String, TestFluid>,
}

/// Describes the test fluid values in the manifest file.
/// This is set of fields that can describe a fluid completly for saturation purposes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TestFluid {
    pub concentration: String,
    pub volume: String,
}

/// Describes the expected results of a test.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Expected {
    pub mixer_sequence: Option<String>,
    pub storage_units: Option<u64>,
}

/// A specific instance of a `TestManifest` from disk.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TestManifestFile {
    pub path: PathBuf,
    pub test_manifest: TestManifest,
}

impl TestManifest {
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let mut warnings = vec![];
        let manifest_str = fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("failed to read manifest at {:?}: {}", path, e))?;
        let toml_de = toml::de::Deserializer::new(&manifest_str);
        let manifest: Self = serde_ignored::deserialize(toml_de, |path| {
            let warning = format!("unused manifest key: {path}");
            warnings.push(warning);
        })
        .map_err(|e| anyhow::anyhow!("failed to parse manifest: {}.", e))?;
        for warning in warnings {
            // TODO: print this yellow.
            println!("WARNING: {}", warning);
        }
        Ok(manifest)
    }
}

impl TestManifestFile {
    /// Read the manifest file from the given path.
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let path = path.canonicalize()?;
        let test_manifest = TestManifest::from_file(&path)?;
        Ok(Self {
            path,
            test_manifest,
        })
    }
}

// TODO: add unit tests
