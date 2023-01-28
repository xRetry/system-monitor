use system_monitor::program::Program;
use system_monitor::program_manager::ProgramManager;

fn main() {
    let mut manager = ProgramManager::new();

    let prog = Program::new("Test".to_owned());
    manager.add_program(prog);
    manager.start_programs();
    manager.check_programs();
}
