use clap::Args;

#[derive(Debug, Args)]
pub struct Command {
    email: String,
    password: String,
}

impl Command {
    pub fn run(&self) {
        println!("Login with email: {} and password: {}", self.email, self.password);
    }
}

