use crate::program::Program;
use std::fs::File;
use std::io::BufReader;
use std::thread;

pub struct ProgramManager {
    programs: Vec<Program>,
}

impl ProgramManager {
    pub fn new() -> ProgramManager {
        return ProgramManager {
            programs: Vec::<Program>::new(),
        };
    }

    pub fn start(&mut self) {
        let file = File::open("config/programs.json").expect("Unable to open file");
        let reader = BufReader::new(file);

        let programs: Vec<Program> =
            serde_json::from_reader(reader).expect("Unable to deserialize json");

        for prog in programs {
            self.add_program(prog);
        }

        self.run();
    }

    fn run(&mut self) {
        loop {
            self.check_programs();
            break;
            thread::sleep(std::time::Duration::from_secs(60 * 5));
        }
    }

    pub fn add_program(&mut self, prog: Program) {
        self.programs.push(prog);
    }

    pub fn check_programs(&mut self) {
        for program in self.programs.iter_mut() {
            program.check(true);
        }
    }
}
