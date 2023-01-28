use crate::program::Program;

pub struct ProgramManager<'a> {
    programs: Vec<Program<'a>>,
}

impl<'a> ProgramManager<'a> {
    pub fn new() -> ProgramManager<'a> {
        return ProgramManager {
            programs: Vec::<Program>::new(),
        };
    }

    pub fn add_program(&mut self, prog: Program<'a>) {
        self.programs.push(prog);
    }

    pub fn start_programs(&mut self) {
        for program in self.programs.iter() {
            program.start();
        }
    }

    pub fn check_programs(&mut self) {
        for program in self.programs.iter() {
            program.check();
        }
    }
}
