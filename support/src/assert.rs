use std::path::PathBuf;

use futures::future;
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

pub async fn assert_file_content(expected: PathBuf, actual: PathBuf) -> tokio::io::Result<()> {
    let expected = fs::read_to_string(expected);
    let actual = fs::read_to_string(actual);
    let (expected, actual) = try_join!(expected, actual)?;

    assert_eq!(
        expected, actual,
        "File content does not match expected content."
    );
    Ok(())
}
