use anyhow::Context;
use chrono::prelude::*;
use futures::future;
use pretty_assertions::assert_eq;
use std::path::PathBuf;
use tokio::{fs, try_join};

pub async fn assert_files_with_file_name(
    expected_dir: &PathBuf,
    actual_files: &Vec<PathBuf>,
) -> anyhow::Result<()> {
    let zipped = actual_files
        .clone()
        .into_iter()
        .map(|file| {
            let expected_file = expected_dir.join(file.file_name().unwrap());
            (expected_file, file)
        })
        .collect::<Vec<_>>();

    let futures = zipped
        .into_iter()
        .map(|(expected, actual)| assert_file_content(expected, actual))
        .collect::<Vec<_>>();

    future::try_join_all(futures).await?;
    Ok(())
}

pub async fn assert_files_with_timestamped_name(
    expected_paths: &Vec<PathBuf>,
) -> anyhow::Result<()> {
    let zipped = expected_paths
        .clone()
        .into_iter()
        .map(|file| {
            let metadata = file.metadata().unwrap();

            let created_at = metadata.created().unwrap();
            let created_at: DateTime<Local> = created_at.into();

            let actual_file =
                file.with_file_name(format!("{}.csv", created_at.format("%Y%m%dT%H%M%S%z")));
            let expected_file = file;

            (expected_file, actual_file)
        })
        .collect::<Vec<_>>();

    let futures = zipped
        .into_iter()
        .map(|(expected, actual)| assert_file_content(expected, actual))
        .collect::<Vec<_>>();

    future::try_join_all(futures).await?;
    Ok(())
}

pub async fn assert_file_content(
    expected_path: PathBuf,
    actual_path: PathBuf,
) -> anyhow::Result<()> {
    let expected = fs::read_to_string(&expected_path);
    let actual = fs::read_to_string(&actual_path);
    let (expected, actual) = try_join!(expected, actual).with_context(|| {
        format!(
            "Failed: expected: {}, actual: {}",
            &expected_path.display(),
            &actual_path.display()
        )
    })?;

    assert_eq!(
        expected, actual,
        "File content does not match expected content."
    );
    Ok(())
}
