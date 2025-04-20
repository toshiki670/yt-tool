extern crate youtube;

use std::time::Duration;
use tempfile::tempdir;
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
    time::sleep,
};
use youtube::prelude::*;

// テスト用の JSON データを返す関数
fn test_json_data() -> String {
    r#"{"replayChatItemAction":{"actions":[{"addChatItemAction":{"item":{"liveChatTextMessageRenderer":{"message":{"runs":[{"text":"テストメッセージ"}]},"authorName":{"simpleText":"テストユーザー"},"timestampUsec":"1700000000000000","id":"test-id-1","authorExternalChannelId":"test-channel-1"}}}}]}}"#.to_string()
}

// WatchChatCommandを直接使用したE2Eテスト
#[tokio::test]
async fn test_live_chat_watch_interface_e2e() -> anyhow::Result<()> {
    use std::path::Path;
    use tokio::sync::oneshot;

    // テンポラリディレクトリを作成
    let test_dir = tempdir()?;
    let file_path = test_dir.path().join("test_watch.live_chat.json.part");

    // 空ファイルを作成
    File::create(&file_path).await?;

    // 出力キャプチャ用のチャネル
    let (tx, _rx) = oneshot::channel::<()>();

    // WatchChatCommandを使用
    let command = WatchChatCommand::new();

    // 標準出力への出力は失敗しても問題ないので、実際のコマンドを使用
    let file_path_clone = file_path.clone();
    let handle = tokio::spawn(async move {
        let result = command.execute(Path::new(&file_path_clone)).await;

        // 完了シグナルを送信
        let _ = tx.send(());

        result
    });

    // 1秒待機してからファイルに書き込む
    sleep(Duration::from_secs(1)).await;

    // テスト用のJSONデータを作成
    let test_json = test_json_data();
    let additional_json = test_json_data().replace("テストメッセージ", "追加メッセージ");

    // ファイルに書き込む
    let mut file = File::create(&file_path).await?;
    let mut writer = BufWriter::new(&mut file);

    writer.write_all(test_json.as_bytes()).await?;
    writer.write_all(b"\n").await?;
    writer.flush().await?;

    // 少し待機
    sleep(Duration::from_secs(1)).await;

    writer.write_all(additional_json.as_bytes()).await?;
    writer.write_all(b"\n").await?;
    writer.flush().await?;

    // さらに待機
    sleep(Duration::from_secs(3)).await;

    // タスクをキャンセル（実際の環境では長時間実行される）
    handle.abort();

    // クリーンアップ
    test_dir.close()?;

    Ok(())
}

// CLIからの利用をシミュレートするテスト
#[tokio::test]
async fn test_live_chat_watch_with_stdout_presenter() -> anyhow::Result<()> {
    use std::path::Path;

    // テンポラリディレクトリを作成
    let test_dir = tempdir()?;
    let file_path = test_dir.path().join("test_cli.live_chat.json.part");

    // 空ファイルを作成
    File::create(&file_path).await?;

    // コマンドを作成
    let command = WatchChatCommand::new();

    // 別タスクでコマンドを実行
    let file_path_clone = file_path.clone();
    let handle = tokio::spawn(async move { command.execute(Path::new(&file_path_clone)).await });

    // 1秒待機してからファイルに書き込む
    sleep(Duration::from_secs(1)).await;

    // テスト用のJSONデータを作成
    let test_json = test_json_data();

    // ファイルに書き込む
    let mut file = File::create(&file_path).await?;
    file.write_all(test_json.as_bytes()).await?;
    file.write_all(b"\n").await?;
    file.flush().await?;

    // 3秒待機
    sleep(Duration::from_secs(3)).await;

    // プロセスを終了
    handle.abort();

    // クリーンアップ
    test_dir.close()?;

    Ok(())
}
