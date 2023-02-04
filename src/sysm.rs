use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Status,
    Program,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Status => println!("Status called"),
        Commands::Program => println!("Program called"),
    }
}
