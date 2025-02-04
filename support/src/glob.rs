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
        .flat_map(|p| p.collect::<Vec<Result<PathBuf, glob::GlobError>>>())
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use tokio::{fs, try_join};

    #[tokio::test]
    async fn test_expend_glob_input_patterns() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let test_dir = temp_dir.path().join("test_expend_glob_input_patterns");
        fs::create_dir_all(&test_dir).await?;
        try_join!(
            fs::File::create(test_dir.join("test.json")),
            fs::File::create(test_dir.join("test2.json")),
        )?;

        let pattern = test_dir.join("*.json");
        let patterns = vec![pattern.to_string_lossy().to_string()];
        let result = expend_glob_input_patterns(&patterns)?;
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], test_dir.join("test.json"));
        assert_eq!(result[1], test_dir.join("test2.json"));

        temp_dir.close()?;
        Ok(())
    }

    #[tokio::test]
    async fn test_not_found_patterns() -> anyhow::Result<()> {
        let patterns = vec!["*.json".to_string()];
        let result = expend_glob_input_patterns(&patterns);

        let expected = ExpendGlobError::NoInputFilesFound.to_string();
        let actual = result.unwrap_err().to_string();
        assert_eq!(expected, actual);
        Ok(())
    }

    #[tokio::test]
    async fn test_not_found_pattern() -> anyhow::Result<()> {
        let pattern = "*.json";
        let result = expend_glob_pattern(pattern);

        let expected = ExpendGlobError::NoInputFilesFound.to_string();
        let actual = result.unwrap_err().to_string();
        assert_eq!(expected, actual);
        Ok(())
    }
}
