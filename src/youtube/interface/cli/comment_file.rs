use clap::ValueHint;
use std::path::PathBuf;

use crate::cli::Route;

#[derive(clap::Args, Debug)]
#[command(name = "Comment File Feature")]
pub(super) struct Args {
    #[clap(value_name = "INPUT FILE", value_hint = ValueHint::FilePath)]
    input_file: PathBuf,

    #[clap(short, long, value_name = "OUTPUT FILE", value_hint = ValueHint::FilePath)]
    output_file: Option<PathBuf>,
}

impl Route for Args {
    fn route(&self) {
        // let value = self.value.clone();
        // println!("{}", value);
    }
}
