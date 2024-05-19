use clap::{Parser, Subcommand};

mod commands;
use crate::commands::{login, wallet, transaction, category, import, sync};

#[derive(Debug, Parser)]
#[command(version = "0.1.0", name = "MoneyLover Rust CLI", about = "Build MoneyLover with Rust", author = "Du Gia Huy <dugiahuy@gmail.com> - exponentdev.com")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Login(login::Command),
    Wallet(wallet::Command),
    Transaction(transaction::Command),
    Category(category::Command),
    Import(import::Command),
    Sync(sync::Command),
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Login(login_cmd) => { login_cmd.run(); } 
        Command::Wallet(wallet_cmd) => { wallet_cmd.run(); } 
        Command::Transaction(transaction_cmd) => { transaction_cmd.run(); } 
        Command::Category(category_cmd) => { category_cmd.run(); } 
        Command::Import(import_cmd) => { import_cmd.run(); } 
        Command::Sync(sync_cmd) => { sync_cmd.run(); } 
    }
}
