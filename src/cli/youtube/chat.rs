// https://www.rustadventure.dev/introducing-clap/clap-v4/accepting-file-paths-as-arguments-in-clap
// https://tech.natsuneko.blog/entry/2022/03/15/exclusive-command-options-in-clap

use crate::cli::Route;
use anyhow::{anyhow, bail};
use clap::ValueEnum;
use glob::glob;
use std::{fmt::Display, path::PathBuf};
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

    #[arg(
        short = 't',
        long,
        default_value = "csv",
        value_name = "FILE_TYPE",
        help = "Output file type"
    )]
    output_type: FileType,
}

#[derive(Clone, Debug, ValueEnum)]
enum FileType {
    Csv,
}

impl Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileType::Csv => write!(f, "csv"),
        }
    }
}

impl Route for Args {
    fn route(&self) -> anyhow::Result<()> {
        // Expand glob patterns and create a list of input files
        let input_files = expend_glob_input_patterns(&self.input_patterns)?;

        let chat_file_service = LiveChatJsonService::new(&input_files);

        match &self.output_type {
            FileType::Csv => {
                chat_file_service.generate_file_with_type(&output_type.to_string())?;
            }
        }

        Ok(())
    }
}

fn expend_glob_input_patterns(input_patterns: &Vec<String>) -> anyhow::Result<Vec<PathBuf>> {
    let mut input_files = Vec::new();
    for pattern in input_patterns {
        let paths = glob(pattern).map_err(|e| anyhow!("Glob pattern is invalid: {}", e))?;

        for entry in paths {
            match entry {
                Ok(path) => input_files.push(path),
                Err(e) => bail!("Warning: Failed to read path: {}", e),
            }
        }
    }
    if input_files.is_empty() {
        bail!("No input files were found");
    }
    Ok(input_files)
}
