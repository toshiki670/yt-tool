extern crate youtube;

use pretty_assertions::assert_eq;
use std::{env, path::PathBuf};
use tempfile::tempdir;
use youtube::prelude::LiveChatJsonInterface;

fn test_root_dir() -> PathBuf {
fn test_json_dir() -> PathBuf {
fn test_root_dir() -> PathBuf {
    env::current_dir()
        .unwrap()
        .join("tests/live_chat_json_service/json/")
}

#[tokio::test]
async fn it_generate_with_path() -> anyhow::Result<()> {
    let temp_dir = tempdir()?;

    let input_path = test_json_dir().join("live_chat.json");
    let output_path = temp_dir.path().join("output.csv");

    let interface = LiveChatJsonInterface::new(&input_path);
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

    let file_name = "live_chat.json";
    let base_input_path = test_root_dir().join(file_name);

    let input_path = temp_dir.path().join(file_name);
    std::fs::copy(&base_input_path, &input_path)?;

    let output_type = "csv".to_string();

    let interface = LiveChatJsonInterface::new(&input_path);
    interface.generate_file_with_type(&output_type).await?;

    let mut to_path = input_path.clone();
    to_path.set_extension(output_type);

    let expected = expected_csv_data_for_test();
    let actual = std::fs::read_to_string(&to_path)?;
    assert_eq!(expected, actual);

    temp_dir.close()?;
    Ok(())
}

fn expected_csv_data_for_test() -> String {
    let mut csvs = Vec::new();

    csvs.push("id,authorExternalChannelId,postedAt,authorName,content,isModerator,membershipMonths,category");
    csvs.push("id,authorExternalChannelId,2024-12-05T12:41:54.906095+09:00,authorName,メッセージ,false,,ChatTextMessage");

    format!("{}\n", csvs.join("\n"))
}
