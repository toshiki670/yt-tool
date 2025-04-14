// https://www.rustadventure.dev/introducing-clap/clap-v4/accepting-file-paths-as-arguments-in-clap
// https://tech.natsuneko.blog/entry/2022/03/15/exclusive-command-options-in-clap

use crate::cli::Route;
use anyhow::Context as _;
use rust_support::glob;
use std::path::PathBuf;
use youtube::prelude::*;

#[derive(clap::Args, Debug)]
#[command(name = "Comment File Feature")]
#[clap(group(
        clap::ArgGroup::new("mode")
            .required(true)
            .args(&["input_patterns", "watch_file"]),
))]
pub(super) struct Args {
    #[clap(
        value_name = "INPUT PATTERNS",
        help = "Input files (glob patterns supported: *.json)",
        group = "mode"
    )]
    input_patterns: Option<Vec<String>>,

    #[clap(
        short = 'r',
        long,
        help = "Rename to include the timestamp",
        requires = "input_patterns"
    )]
    rename_with_timestamp: bool,

    #[clap(
        short = 'w',
        long = "watch",
        value_name = "FILE_PATH",
        help = "Watch the specified file for changes and output new lines",
        group = "mode"
    )]
    watch_file: Option<PathBuf>,
}

impl Route for Args {
    async fn route(&self) -> anyhow::Result<()> {
        if let Some(file_path) = &self.watch_file {
            watch_and_print(file_path)
                .await
                .context("Chat watching process failed")?;
        } else if let Some(patterns) = &self.input_patterns {
            let input_files = glob::expend_glob_input_patterns(patterns)?;
            let interface = LiveChatJsonInterface::new(&input_files);

            if self.rename_with_timestamp {
                interface.generate_files_with_timestamped_name().await?;
            } else {
                interface.generate_files_with_csv().await?;
            }
        } else {
            anyhow::bail!("Either input patterns or a file to watch must be specified.");
        }

        Ok(())
    }
}
