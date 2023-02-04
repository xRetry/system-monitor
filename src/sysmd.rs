use sysmon::{sysmon_server::SysMonServer, SysMon, StatusResponse};
use system_monitor::program_manager::ProgramManager;
use tonic::{transport::Server, Status, Response};

pub mod sysmon {
    tonic::include_proto!("sysmon");
}

#[derive(Debug, Default)]
struct SysMonDaemon {}

#[tonic::async_trait]
impl SysMon for SysMonDaemon {
    async fn check_status(&self) -> Response<StatusResponse> {
        Response::new(StatusResponse {
            error_message: "",
            uptime_sec: 10,
            running_programs: 5,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:8080".parse().unwrap();
    let daemon = SysMonDaemon::default();

    Server::builder().add_service(SysMonServer::new(daemon))
        .serve(addr)
        .await?;

    let mut manager = ProgramManager::new();
    manager.start();

    Ok(())
}
