use clap::Args;

#[derive(Debug, Args)]
pub struct Command {

}

impl Command {
    pub fn run(&self) {
        println!("Import");
    }
    
}