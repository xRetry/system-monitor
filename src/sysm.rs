use clap::{Parser, Subcommand};
use sysmon::sysmon_client::SysMonClient;

pub mod sysmon {
    tonic::include_proto!("sysmon");
}

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut client = SysMonClient::connect("http://[::1]:8080").await?;

    match &cli.command {
        Commands::Status => {
            println!("Status called");
            let response = client.check_status().await?;
            println!("{}", response.into_inner().error_message)
        }
        Commands::Program => println!("Program called"),
    }
    Ok(())
}
