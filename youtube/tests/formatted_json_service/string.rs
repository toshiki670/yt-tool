extern crate youtube;

use pretty_assertions::assert_eq;
use tempfile::tempdir;
use youtube::prelude::FormattedJsonService;

#[test]
fn it_generate_with_path() -> anyhow::Result<()> {
    let temp_dir = tempdir()?;

    let input_json = test_formatted_json_data();
    let output_path = temp_dir.path().join("output.csv");

    let chat_file_service = FormattedJsonService::new(&input_json);
    chat_file_service.generate_file_with_path(&output_path)?;

    let expected = expected_csv_data_for_test();
    let actual = std::fs::read_to_string(&output_path)?;
    assert_eq!(expected, actual);

    temp_dir.close()?;
    Ok(())
}

fn expected_csv_data_for_test() -> String {
    let mut csvs = Vec::new();

    csvs.push("id,authorExternalChannelId,postedAt,authorName,content,isModerator,membershipMonths,category");

    format!("{}\n", csvs.join("\n"))
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
