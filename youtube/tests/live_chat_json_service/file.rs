extern crate youtube;

use anyhow::Context;
use chrono::prelude::*;
use futures::future;
use std::{
    env,
    fs::File,
    io::{BufRead as _, BufReader},
    path::{Path, PathBuf},
};
use tempfile::tempdir;
use tokio::fs;
use youtube::prelude::LiveChatJsonInterface;

fn test_root_dir() -> PathBuf {
    env::current_dir()
        .unwrap()
        .join("tests/live_chat_json_service/")
}

fn test_json_dir() -> PathBuf {
    test_root_dir().join("json/")
}

fn test_expected_dir() -> PathBuf {
    test_root_dir().join("expected/")
}

// #[tokio::test]
// async fn it_generate_with_paths() -> anyhow::Result<()> {
//     let test_dir = tempdir()?;

//     // Copy test json files
//     let test_json_dir = test_json_dir();
//     let source = test_json_dir.join("*");
//     cp(&source.to_string_lossy(), test_dir.path()).await?;

//     // Read test json files
//     let imput_paths = read_paths(test_dir.path())?;

//     // Run the test subject
//     let interface = LiveChatJsonInterface::new(&imput_paths);
//     interface.generate_files_with_csv().await?;

//     // Prepare expected file paths
//     let expected_dir = test_expected_dir();

//     // Prepare actual file paths
//     let actual_files = imput_paths
//         .iter()
//         .map(|path| path.with_extension("csv"))
//         .collect::<Vec<_>>();

//     // Assert the result
//     assert_files_with_file_name(&expected_dir, &actual_files).await?;

//     test_dir.close()?;
//     Ok(())
// }

#[tokio::test]
async fn it_generate_with_path_and_timestamped_name() -> anyhow::Result<()> {
    let test_dir = tempdir()?;

    // Copy test json files
    let test_json_dir = test_json_dir();
    let source = test_json_dir.join("*");
    cp(&source.to_string_lossy(), test_dir.path()).await?;

    // Read test json file
    let imput_path = test_dir.path().join("live_chat.json");
    let imput_paths = vec![imput_path];

    // Run the test subject
    let interface = LiveChatJsonInterface::new(&imput_paths);
    interface.generate_files_with_timestamped_name().await?;

    // Prepare expected file paths
    let imput_path = imput_paths.first().unwrap();
    let metadata = imput_path.metadata().unwrap();

    let created_at = metadata.created().unwrap();
    let created_at: DateTime<Local> = created_at.into();

    let expected_paths =
        imput_path.with_file_name(format!("{}.csv", created_at.format("%Y-%m-%dT%H-%M-%S%z")));

    let exists = fs::try_exists(&expected_paths).await?;

    // Assert that the file was created by timestamped name
    assert!(exists);

    test_dir.close()?;
    Ok(())
}

#[tokio::test]
async fn it_generate_with_type() -> anyhow::Result<()> {
    let test_dir = tempdir()?;

    // Copy test json file
    let file_name = "live_chat.json";
    let base_input_path = test_json_dir().join(file_name);
    let input_path = test_dir.path().join(file_name);
    fs::copy(base_input_path, &input_path).await?;

    let output_type = "csv".to_string();

    // Run the test subject
    let interface = LiveChatJsonInterface::new(&input_path);
    interface.generate_file_with_type(&output_type).await?;

    // Assert the result
    let expected_path = test_expected_dir().join("live_chat.csv");
    let mut last_line = None;
    let reader = BufReader::new(File::open(expected_path)?);

    for result in reader.lines() {
        let record = result?;
        last_line = Some(record);
    }

    if let Some(last_line) = last_line {
        assert_eq!(last_line, "id,authorExternalChannelId,2024-12-05T12:41:54.906095+09:00,Text message,authorName,message: 'メッセージ'");
    }

    test_dir.close()?;
    Ok(())
}

async fn cp(src_pattern: &str, dst_dir: &Path) -> anyhow::Result<()> {
    let src_paths = rust_support::glob::expend_glob_pattern(src_pattern)?;

    let sets = src_paths
        .into_iter()
        .map(|src| {
            let name = src
                .file_name()
                .with_context(|| format!("Failed to get file name from {}", src.display()))?;
            let dst = dst_dir.join(name);

            Ok((src, dst))
        })
        .collect::<Vec<anyhow::Result<(PathBuf, PathBuf)>>>();

    let sets = rust_support::anyhow::collect_results(sets)?;

    let futures = sets
        .into_iter()
        .map(|set| {
            let (src, dst) = set;
            fs::copy(src, dst)
        })
        .collect::<Vec<_>>();

    future::try_join_all(futures).await?;

    Ok(())
}

// fn read_paths(path: &Path) -> anyhow::Result<Vec<PathBuf>> {
//     let dir = std::fs::read_dir(path)?;

//     let results = dir.map(|e| e.map_err(|e| anyhow::anyhow!(e))).collect();
//     let dir_entries = support::anyhow::collect_results(results)?;
//     let paths = dir_entries.iter().map(|e| e.path()).collect();

//     Ok(paths)
// }
