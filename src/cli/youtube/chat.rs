// https://www.rustadventure.dev/introducing-clap/clap-v4/accepting-file-paths-as-arguments-in-clap
// https://tech.natsuneko.blog/entry/2022/03/15/exclusive-command-options-in-clap

use crate::cli::Route;
use clap::{ArgGroup, ValueEnum, ValueHint};
use std::{fmt::Display, path::PathBuf};
use youtube::prelude::ChatFileService;

#[derive(clap::Args, Debug)]
#[command(name = "Comment File Feature")]
#[clap(group(ArgGroup::new("output")
    .args(&["output_file"])
    .conflicts_with("output_type"))
)]
pub(super) struct Args {
    #[clap(value_name = "INPUT FILE", value_hint = ValueHint::FilePath)]
    input_file: PathBuf,

    #[clap(short, long, value_name = "OUTPUT FILE", value_hint = ValueHint::FilePath)]
    output_file: Option<PathBuf>,

    #[arg(short, long, value_name = "FILE_TYPE", help = "Output file type")]
    output_type: Option<FileType>,
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
        let input_file = &self.input_file;
        let chat_file_service = ChatFileService::new(input_file);

        if let Some(output_file) = &self.output_file {
            chat_file_service.convert_with_path(output_file)?;
        } else if let Some(output_type) = &self.output_type {
            chat_file_service.convert_with_type(&output_type.to_string())?;
        } else {
            unreachable!();
        }

        Ok(())
    }
}
