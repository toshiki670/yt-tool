use crate::cli::Route;

#[derive(clap::Args, Debug)]
#[command(name = "Comment File Feature")]
pub struct Args {
    pub value: String,
}

impl Route for Args {
    fn route(&self) {
        let value = self.value.clone();
        println!("{}", value);
    }
}
