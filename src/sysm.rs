use clap::{Parser, Subcommand};
use sysmon::{sys_mon_client::SysMonClient, Empty};
use tonic::Request;

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
            let response = client.check_status(Request::new(Empty{})).await?
                .into_inner();

            println!("{}", response.error_message);
            println!("{}", response.running_programs);
        }
        Commands::Program => println!("Program called"),
    }
    Ok(())
}
