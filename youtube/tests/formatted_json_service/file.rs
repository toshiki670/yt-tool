extern crate youtube;

use pretty_assertions::assert_eq;
use std::{env, path::PathBuf};
use support::assert::assert_file_content;
use tempfile::tempdir;
use tokio::fs;
use youtube::prelude::FormattedJsonInterface;

fn test_root_dir() -> PathBuf {
    env::current_dir()
        .unwrap()
        .join("tests/formatted_json_service/")
}

fn test_json_dir() -> PathBuf {
    test_root_dir().join("json/")
}

fn test_expected_dir() -> PathBuf {
    test_root_dir().join("expected/")
}

#[tokio::test]
async fn it_generate_with_path() -> anyhow::Result<()> {
    let temp_dir = tempdir()?;

    let input_path = test_json_dir().join("formatted.json");
    let output_path = temp_dir.path().join("output.csv");

    let interface = FormattedJsonInterface::new(&input_path);
    interface.generate_file_with_path(&output_path).await?;

    let expected = expected_csv_data_for_test();
    let actual = std::fs::read_to_string(&output_path)?;
    assert_eq!(expected, actual);

    temp_dir.close()?;
    Ok(())
}

#[tokio::test]
async fn it_generate_with_type() -> anyhow::Result<()> {
    let temp_dir = tempdir()?;

    // Create test directory
    let test_dir = temp_dir.path().join("it_generate_with_type");
    fs::create_dir_all(&test_dir).await?;

    // Copy test json file
    let file_name = "formatted.json";
    let base_input_path = test_json_dir().join(file_name);
    let input_path = temp_dir.path().join(file_name);
    fs::copy(&base_input_path, &input_path).await?;

    let output_type = "csv".to_string();

    // Run the test subject
    let interface = FormattedJsonInterface::new(&input_path);
    interface.generate_file_with_type(&output_type).await?;

    // Assert the result
    let expected_path = test_expected_dir().join("formatted.csv");
    let mut to_path = input_path.clone();
    to_path.set_extension(output_type);
    assert_file_content(expected_path, to_path).await?;

    temp_dir.close()?;
    Ok(())
}

fn expected_csv_data_for_test() -> String {
    let mut csvs = Vec::new();

    csvs.push("id,authorExternalChannelId,postedAt,authorName,content,isModerator,membershipMonths,category");
    csvs.push("id,authorExternalChannelId,2024-12-05T12:41:54.906095+09:00,authorName,メッセージ,false,,ChatTextMessage");

    format!("{}\n", csvs.join("\n"))
}
