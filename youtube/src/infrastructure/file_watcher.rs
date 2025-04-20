use crate::application::ports::new_line_provider::{NewLineProvider, ProviderError};
use anyhow::Result;
use async_stream::try_stream;
use async_trait::async_trait;
use log::{debug, error, info};
use notify::event::{DataChange, ModifyKind};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::pin::Pin;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncSeekExt, BufReader, SeekFrom};
use tokio::time::interval; // Using interval for non-blocking check
use tokio_stream::Stream; // StreamExt as TokioStreamExt を削除

/// ファイル監視を行い、NewLineProvider ポートを実装するアダプター
#[derive(Default, Clone)] // Default を追加 (インスタンス化しやすくするため)
pub struct FileWatcherAdapter;

#[async_trait]
impl NewLineProvider for FileWatcherAdapter {
    async fn watch_new_lines(
        &self,
        file_path: &Path,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String, ProviderError>> + Send>>, ProviderError>
    {
        let file_path_buf = file_path.to_path_buf();
        info!("Initializing file watcher for: {}", file_path_buf.display());

        // --- Event Handling Setup ---
        let (event_tx, event_rx) = tokio::sync::mpsc::channel::<()>(1); // Use tokio mpsc
        let path_for_watcher = file_path_buf.clone();

        // Run the synchronous file watcher in a separate thread
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to create Tokio runtime for watcher thread");

            rt.block_on(async move {
                let event_tx_clone = event_tx.clone();
                let mut watcher = match RecommendedWatcher::new(
                    move |res: Result<notify::Event, notify::Error>| {
                        if let Ok(event) = res {
                            if matches!(
                                event.kind,
                                EventKind::Modify(ModifyKind::Data(DataChange::Content))
                            ) || matches!(event.kind, EventKind::Modify(ModifyKind::Name(_)))
                            {
                                debug!("Detected file change event: {event:?}");
                                // Send event non-blockingly
                                let _ = event_tx_clone.try_send(());
                            }
                        }
                    },
                    Config::default(),
                ) {
                    Ok(w) => w,
                    Err(e) => {
                        error!("Failed to create file watcher: {e}");
                        return;
                    }
                };

                if let Err(e) = watcher.watch(&path_for_watcher, RecursiveMode::NonRecursive) {
                    error!(
                        "Failed to start watching file {}: {}",
                        path_for_watcher.display(),
                        e
                    );
                    return;
                }
                info!(
                    "Watcher started successfully in dedicated thread for: {}",
                    path_for_watcher.display()
                );

                // Keep the thread alive while the watcher is running
                // This is a simple way; could use a signal channel for shutdown
                loop {
                    tokio::time::sleep(Duration::from_secs(60)).await;
                }
            });
        });

        // --- Stream Generation ---
        let stream = try_stream! {
            let mut file = File::open(&file_path_buf).await?;
            file.seek(SeekFrom::End(0)).await?; // Start from the end
            let mut reader = BufReader::new(file);
            let mut line_buffer = String::new();
            let mut interval = interval(Duration::from_millis(200)); // Check interval
            let mut event_rx = event_rx; // Move receiver into the stream state

            loop {
                tokio::select! {
                    // Wait for either a file change event or the interval tick
                    _ = event_rx.recv() => {
                        debug!("Received file change event notification.");
                        // Drain any other queued events quickly
                        while event_rx.try_recv().is_ok() {}
                    }
                    _ = interval.tick() => {
                        // Interval tick, proceed to read
                        debug!("Interval tick, checking for new lines.");
                    }
                }

                // Try reading new lines
                loop {
                    line_buffer.clear();
                    match reader.read_line(&mut line_buffer).await {
                        Ok(0) => break, // EOF for now
                        Ok(_) => {
                            if !line_buffer.trim().is_empty() {
                                debug!("Yielding new line: {}", line_buffer.trim());
                                yield line_buffer.trim_end().to_string();
                            }
                        }
                        Err(e) => {
                            error!("Error reading line from {}: {}", file_path_buf.display(), e);
                            // 条件を改善: 存在するかの確認を先に行う
                            if file_path_buf.exists() {
                                // Yield the IO error but continue trying
                                Err(ProviderError::Io(e))?;
                            } else {
                                error!("Watched file no longer exists: {}", file_path_buf.display());
                                Err(ProviderError::Io(std::io::Error::new(
                                    std::io::ErrorKind::NotFound,
                                    "Watched file disappeared"
                                )))?;
                            }
                        }
                    }
                }
            }
        };

        Ok(Box::pin(stream))
    }
}
