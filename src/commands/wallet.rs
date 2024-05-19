use clap::Args;

#[derive(Debug, Args)]
pub struct Command {
  #[arg(short = 'a', long = "all")]
  all: bool,
  wallet: Option<String>,
}

impl Command {
  pub fn run(&self) {
    println!("List wallet with all: {:?} and wallet: {:?}", self.all, self.wallet);
  }
}