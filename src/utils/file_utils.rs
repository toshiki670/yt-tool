use anyhow::{anyhow, bail};
use glob::glob;
use std::path::PathBuf;

pub(crate) fn expend_glob_input_patterns(
    input_patterns: &Vec<String>,
) -> anyhow::Result<Vec<PathBuf>> {
    let mut input_files = Vec::new();
    for pattern in input_patterns {
        let paths = glob(pattern).map_err(|e| anyhow!("Glob pattern is invalid: {}", e))?;

        for entry in paths {
            match entry {
                Ok(path) => input_files.push(path),
                Err(e) => bail!("Warning: Failed to read path: {}", e),
            }
        }
    }
    if input_files.is_empty() {
        bail!("No input files were found");
    }
    Ok(input_files)
}
