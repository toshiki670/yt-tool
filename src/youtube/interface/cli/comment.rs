use crate::cli::Route;
use clap::Args;

#[derive(Args, Debug)]
#[command(name = "Comment")]
pub struct CommentArgs {
    pub value: String,
}

impl Route for CommentArgs {
    fn route(&self) {
        let value = self.value.clone();
        println!("{}", value);
    }
}
