use system_monitor::program_manager::ProgramManager;
use tonic::{transport::Server, Status, Response, Request};
use std::sync::Arc;

pub mod sysmon {
    tonic::include_proto!("sysmon");
}

use sysmon::{sys_mon_server::{SysMonServer, SysMon}, Empty, StatusResponse};


struct SysMonDaemon {
    prog_manager: Arc<ProgramManager>
}

#[tonic::async_trait]
impl SysMon for SysMonDaemon {
    async fn check_status(&self, _: Request<Empty>) -> Result<Response<StatusResponse>, Status> {
        Ok(Response::new(StatusResponse {
            error_message: "".to_string(),
            uptime_sec: 10,
            running_programs: 5,
        }))
    }
}

async fn handle_rpc(daemon: SysMonDaemon) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:8080".parse().unwrap();
    //let daemon = SysMonDaemon::default();

    Server::builder().add_service(SysMonServer::new(daemon))
        .serve(addr)
        .await?;
    Ok(())
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = Arc::new(ProgramManager::new());

    let daemon = SysMonDaemon{
        prog_manager: manager.clone() 
    };
    let rpc_thread_handle = std::thread::spawn(|| {
        handle_rpc(daemon);
    });

    println!("Starting manager");
    manager.start();

    let _ = rpc_thread_handle.join();

    Ok(())
}
