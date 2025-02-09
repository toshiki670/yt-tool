extern crate youtube;

use pretty_assertions::assert_eq;
use std::{env, path::PathBuf};
use tempfile::tempdir;
use tokio::{fs, try_join};
use youtube::prelude::FormattedJsonInterface;

fn test_root_dir() -> PathBuf {
    env::current_dir()
        .unwrap()
        .join("tests/formatted_json_service/")
}

fn test_expected_dir() -> PathBuf {
    test_root_dir().join("expected/")
}

#[tokio::test]
async fn it_generate_with_path() -> anyhow::Result<()> {
    let test_dir = tempdir()?;

    let input_json = test_formatted_json_data();
    let output_path = test_dir.path().join("output.csv");

    // Run the test subject
    let interface = FormattedJsonInterface::new(&input_json);
    interface.generate_file_with_path(&output_path).await?;

    // Read expected csv file
    let expected_path = test_expected_dir().join("formatted.csv");
    let expected = fs::read_to_string(expected_path);

    // Read actual csv file
    let actual = fs::read_to_string(&output_path);

    // Assert the result
    let (expected, actual) = try_join!(expected, actual)?;
    assert_eq!(expected, actual);

    test_dir.close()?;
    Ok(())
}

fn test_formatted_json_data() -> String {
    let s = r#"
    {
    "replayChatItemAction": {
        "actions": [
        {
            "clickTrackingParams": "clickTrackingParams",
            "addChatItemAction": {
            "item": {
                "liveChatTextMessageRenderer": {
                "message": { "runs": [{ "text": "メッセージ" }] },
                "authorName": { "simpleText": "authorName" },
                "authorPhoto": {
                    "thumbnails": [
                    {
                        "url": "https://yt4.ggpht.com/",
                        "width": 32,
                        "height": 32
                    },
                    { "url": "https://yt4.ggpht.com/", "width": 64, "height": 64 }
                    ]
                },
                "contextMenuEndpoint": {
                    "clickTrackingParams": "clickTrackingParams",
                    "commandMetadata": {
                    "webCommandMetadata": { "ignoreNavigation": true }
                    },
                    "liveChatItemContextMenuEndpoint": { "params": "params==" }
                },
                "id": "id",
                "timestampUsec": "1733370114906095",
                "authorExternalChannelId": "authorExternalChannelId",
                "contextMenuAccessibility": {
                    "accessibilityData": { "label": "Chat actions" }
                },
                "trackingParams": "trackingParams"
                }
            },
            "clientId": "clientId"
            }
        }
        ]
    },
    "videoOffsetTimeMsec": "-416809",
    "isLive": true
    }
    "#;

    s.to_string()
}
