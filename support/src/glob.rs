use crate::anyhow::collect_results;
use glob::glob;
use std::path::PathBuf;
use thiserror::Error;

// Summary:
// 1. Convert patterns to glob results
// 2. Collect results into a single array
// 3. Convert iterator Paths to an array
// 4. Fold arrays into a single array
// 5. Collect results into a single array
// 6. Return the result
pub fn expend_glob_input_patterns(patterns: &[String]) -> anyhow::Result<Vec<PathBuf>> {
    let results = patterns
        .iter()
        .map(|s| glob(s))
        .map(|r| r.map_err(|e| anyhow::anyhow!(e)))
        .collect();

    let paths_array = collect_results(results).map_err(ExpendGlobError::InvalidPatterns)?;

    let results = paths_array
        .into_iter()
        // Convert iterator Paths to an array
        .map(|p| p.collect::<Vec<Result<PathBuf, glob::GlobError>>>())
        .flatten()
        .map(|p| p.map_err(|e| anyhow::anyhow!(e)))
        .collect();

    let path_bufs = collect_results(results).map_err(ExpendGlobError::InvalidGlob)?;

    if path_bufs.is_empty() {
        anyhow::bail!(ExpendGlobError::NoInputFilesFound);
    }

    Ok(path_bufs)
}

pub fn expend_glob_pattern(pattern: &str) -> anyhow::Result<Vec<PathBuf>> {
    let paths = glob(pattern).map_err(ExpendGlobError::InvalidPattern)?;

    let results = paths.map(|p| p.map_err(|e| anyhow::anyhow!(e))).collect();
    let path_bufs = collect_results(results).map_err(ExpendGlobError::InvalidGlob)?;

    if path_bufs.is_empty() {
        anyhow::bail!(ExpendGlobError::NoInputFilesFound);
    }

    Ok(path_bufs)
}

#[derive(Error, Debug)]
pub enum ExpendGlobError {
    #[error("invalid pattern: {0}")]
    InvalidPattern(glob::PatternError),

    #[error("invalid patterns: {0}")]
    InvalidPatterns(anyhow::Error),

    #[error("invalid globs: {0}")]
    InvalidGlob(anyhow::Error),

    #[error("no input files were found")]
    NoInputFilesFound,
}
