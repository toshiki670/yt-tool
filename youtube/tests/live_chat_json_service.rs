extern crate youtube;

mod file {
    use pretty_assertions::assert_eq;
    use std::env;
    use tempfile::tempdir;
    use youtube::prelude::LiveChatJsonService;

    #[test]
    fn it_generate_with_path() -> anyhow::Result<()> {
        let current_path = env::current_dir()?;
        let temp_dir = tempdir()?;

        let input_path = current_path.join("tests/json/live_chat.json");
        // let input_path = current_path.join("../testxxx.json");
        let output_path = temp_dir.path().join("output.csv");

        let chat_file_service = LiveChatJsonService::new(&input_path);
        chat_file_service.generate_file_with_path(&output_path)?;

        let expected = expected_csv_data_for_test();
        let actual = std::fs::read_to_string(&output_path)?;
        assert_eq!(expected, actual);

        temp_dir.close()?;
        Ok(())
    }

    #[test]
    fn it_generate_with_type() -> anyhow::Result<()> {
        let current_path = env::current_dir()?;
        let temp_dir = tempdir()?;

        let file_name = "live_chat.json";
        let base_input_path = current_path.join("tests/json/").join(file_name);

        let input_path = temp_dir.path().join(file_name);
        std::fs::copy(&base_input_path, &input_path)?;

        let output_type = "csv".to_string();

        let chat_file_service = LiveChatJsonService::new(&input_path);
        chat_file_service.generate_file_with_type(&output_type)?;

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
}
