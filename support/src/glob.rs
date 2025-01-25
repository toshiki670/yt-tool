use crate::anyhow::collect_results;
use glob::glob;
use std::path::PathBuf;
use thiserror::Error;

pub fn expend_glob_input_patterns(patterns: &Vec<String>) -> anyhow::Result<Vec<PathBuf>> {
    let results = patterns
        .iter()
        .map(|p| glob(p))
        .map(|p| p.map_err(|e| anyhow::anyhow!(e)))
        .collect();

    let globs = collect_results(results).map_err(|e| ExpendGlobError::InvalidPatterns(e))?;

    let results = globs
        .into_iter()
        // Convert iterator Paths to an array
        .map(|p| p.collect::<Vec<Result<PathBuf, glob::GlobError>>>())
        // Fold arrays into a single array
        .fold(vec![], |mut acc, g| {
            acc.extend(g);
            acc
        })
        .into_iter()
        .map(|p| p.map_err(|e| anyhow::anyhow!(e)))
        .collect();

    let path_bufs = collect_results(results).map_err(|e| ExpendGlobError::InvalidGlob(e))?;

    if path_bufs.is_empty() {
        anyhow::bail!(ExpendGlobError::NoInputFilesFound);
    }

    Ok(path_bufs)
}

#[derive(Error, Debug)]
pub enum ExpendGlobError {
    #[error("invalid patterns: {0}")]
    InvalidPatterns(anyhow::Error),

    #[error("invalid globs: {0}")]
    InvalidGlob(anyhow::Error),

    #[error("no input files were found")]
    NoInputFilesFound,
}
