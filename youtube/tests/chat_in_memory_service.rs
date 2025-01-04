extern crate youtube;

mod generate_from_lines {
    use pretty_assertions::assert_eq;
    use youtube::prelude::ChatInMemoryService;

    #[test]
    fn it_generates_simple_chat_csv_data_from_live_chat_json_data_with_in_memory(
    ) -> anyhow::Result<()> {
        let expected = expected_csv_data_for_test();

        let input_json_data = live_chat_json_data_for_test();
        let actual = ChatInMemoryService::generate_from_lines(input_json_data)?;

        assert_eq!(expected, actual);

        Ok(())
    }

    fn live_chat_json_data_for_test() -> String {
        let mut jsons = Vec::new();

        // live_chat_text_message_renderer
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
}
