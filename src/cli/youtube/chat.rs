// https://www.rustadventure.dev/introducing-clap/clap-v4/accepting-file-paths-as-arguments-in-clap
// https://tech.natsuneko.blog/entry/2022/03/15/exclusive-command-options-in-clap

use crate::cli::Route;
use crate::utils::file_utils::expend_glob_input_patterns;
use youtube::prelude::*;

#[derive(clap::Args, Debug)]
#[command(name = "Comment File Feature")]
pub(super) struct Args {
    #[clap(
        value_name = "INPUT PATTERNS",
        help = "Input files (glob patterns supported: *.json)"
    )]
    input_patterns: Vec<String>,

    #[clap(short = 'r', long, help = "Rename to include the timestamp")]
    rename_with_timestamp: bool,
}

impl Route for Args {
    async fn route(&self) -> anyhow::Result<()> {
        // Expand glob patterns and create a list of input files
        let input_files = expend_glob_input_patterns(&self.input_patterns)?;

        let chat_file_service = LiveChatJsonService::new(&input_files);

        if self.rename_with_timestamp {
            chat_file_service.generate_files_with_csv().await?;
        } else {
            chat_file_service.generate_files_with_csv().await?;
        }

        Ok(())
    }
}
