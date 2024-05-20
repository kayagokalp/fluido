use std::{
    future::Future,
    io::Read,
    path::{Path, PathBuf},
};

use crate::manifest::TestManifestFile;
use regex::{Captures, Regex};

/// Default name of the test manifest file.
const TEST_MANIFEST_FILE: &str = "test.toml";

pub(crate) async fn run_and_capture_output<F, Fut, T>(func: F) -> (T, String)
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = T>,
{
    let mut output = String::new();

    // Capture both stdout and stderr to buffers, run the code and save to a string.
    let buf_stdout = gag::BufferRedirect::stdout();
    let buf_stderr = gag::BufferRedirect::stderr();
    let result = func().await;

    if let Ok(mut buf_stdout) = buf_stdout {
        buf_stdout.read_to_string(&mut output).unwrap();
        drop(buf_stdout);
    }

    if let Ok(mut buf_stderr) = buf_stderr {
        buf_stderr.read_to_string(&mut output).unwrap();
        drop(buf_stderr);
    }

    if cfg!(windows) {
        // In windows output error and warning path files start with \\?\
        // We replace \ by / so tests can check unix paths only
        let regex = Regex::new(r"\\\\?\\(.*)").unwrap();
        output = regex
            .replace_all(output.as_str(), |caps: &Captures| {
                caps[1].replace('\\', "/")
            })
            .to_string();
    }

    (result, output)
}

/// Starting from the `e2e-tests/src/tests/` discovers all the tests we have and collects their manifest files to be processed later on.
pub fn discover_test_configs() -> anyhow::Result<Vec<TestManifestFile>> {
    fn recursive_search(path: &Path, configs: &mut Vec<TestManifestFile>) -> anyhow::Result<()> {
        let wrap_err = |e| {
            let relative_path = path
                .iter()
                .skip_while(|part| part.to_string_lossy() != "test_programs")
                .skip(1)
                .collect::<PathBuf>();
            anyhow::anyhow!("{}: {}", relative_path.display(), e)
        };
        if path.is_dir() {
            for entry in std::fs::read_dir(path).unwrap() {
                recursive_search(&entry.unwrap().path(), configs)?;
            }
        } else if path.is_file()
            && path
                .file_name()
                .map(|f| f == TEST_MANIFEST_FILE)
                .unwrap_or(false)
        {
            let test_toml = TestManifestFile::from_file(path).map_err(wrap_err)?;
            configs.push(test_toml);
        }
        Ok(())
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let tests_root_dir = format!("{manifest_dir}/src/tests");

    let mut configs = Vec::new();
    recursive_search(&PathBuf::from(tests_root_dir), &mut configs)?;
    Ok(configs)
}

pub trait VecExt<T> {
    fn retained<F>(&mut self, f: F) -> Vec<T>
    where
        F: FnMut(&T) -> bool;
}

impl<T> VecExt<T> for Vec<T> {
    fn retained<F>(&mut self, mut f: F) -> Vec<T>
    where
        F: FnMut(&T) -> bool,
    {
        let mut removed = Vec::new();
        let mut i = 0;
        while i < self.len() {
            if f(&mut self[i]) {
                i += 1;
            } else {
                let val = self.remove(i);
                removed.push(val);
            }
        }
        removed
    }
}
