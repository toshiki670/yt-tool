extern crate youtube;

use pretty_assertions::assert_eq;
use tempfile::tempdir;
use youtube::prelude::ChatFileService;

#[test]
fn it_generates_simple_chat_csv_data_from_live_chat_json_data_with_path() -> anyhow::Result<()> {
    let temp_dir = tempdir()?;

    let input_path = temp_dir.path().join("input.json");
    let output_path = temp_dir.path().join("output.csv");

    std::fs::write(&input_path, live_chat_json_data_for_test())?;

    let chat_file_service = ChatFileService::new(&input_path);
    chat_file_service.generate_with_path(&output_path)?;

    let expected = expected_csv_data_for_test();
    let actual = std::fs::read_to_string(&output_path)?;

    assert_eq!(expected, actual);

    temp_dir.close()?;
    Ok(())
}

fn live_chat_json_data_for_test() -> String {
    let mut jsons = Vec::new();

    jsons.push(
        r#"{"replayChatItemAction":{"actions":[{"clickTrackingParams":"clickTrackingParams","addChatItemAction":{"item":{"liveChatTextMessageRenderer":{"message":{"runs":[{"text":"メッセージ"}]},"authorName":{"simpleText":"authorName"},"authorPhoto":{"thumbnails":[{"url":"https://yt4.ggpht.com/","width":32,"height":32},{"url":"https://yt4.ggpht.com/","width":64,"height":64}]},"contextMenuEndpoint":{"clickTrackingParams":"clickTrackingParams","commandMetadata":{"webCommandMetadata":{"ignoreNavigation":true}},"liveChatItemContextMenuEndpoint":{"params":"params=="}},"id":"id","timestampUsec":"1733370114906095","authorExternalChannelId":"authorExternalChannelId","contextMenuAccessibility":{"accessibilityData":{"label":"Chat actions"}},"trackingParams":"trackingParams"}},"clientId":"clientId"}}]},"videoOffsetTimeMsec":"-416809","isLive":true}"#
    );

    jsons.join("\n")
}

fn expected_csv_data_for_test() -> String {
    let mut csvs = Vec::new();

    csvs.push("id,authorExternalChannelId,postedAt,authorName,content,isModerator,membershipMonths,category");
    csvs.push("id,authorExternalChannelId,2024-12-05T12:41:54.906095+09:00,authorName,メッセージ,false,,ChatTextMessage");

    format!("{}\n", csvs.join("\n"))
}
