use crate::cli::Route;

#[derive(clap::Args, Debug)]
#[command(name = "Comment File Feature")]
pub(super) struct Args {
    value: String,
}

impl Route for Args {
    fn route(&self) {
        let value = self.value.clone();
        println!("{}", value);
    }
}
