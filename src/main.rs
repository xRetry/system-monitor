use system_monitor::program_manager::ProgramManager;

fn main() {
    let mut manager = ProgramManager::new();
    manager.init();
    manager.check_programs();
}
