use std::process::{Command, Output, Stdio};
use std::string::FromUtf8Error;

#[derive(Debug)]
struct CommandParseError {}

struct Program<'a> {
    name: String,
    cmd_start: &'a str,
    cmd_status: &'a str,
    expected_start: Option<&'a str>,
    expected_status: Option<&'a str>,
}

impl<'a> Program<'a> {
    pub fn new(name: String) -> Program<'a> {
        return Program {
            name: name,
            cmd_start: "ls -la",
            cmd_status: "echo hello",
            expected_start: None,
            expected_status: Some("hello"),
        };
    }

    pub fn check(&self) -> Result<bool, FromUtf8Error> {
        let is_ok = self.compare_to_expected(self.cmd_status, self.expected_status)?;
        return Ok(is_ok);
    }

    pub fn start(&self) -> Result<bool, FromUtf8Error> {
        let is_ok = self.compare_to_expected(self.cmd_start, self.expected_start)?;
        return Ok(is_ok);
    }

    fn compare_to_expected(
        &self,
        cmd: &str,
        expected: Option<&str>,
    ) -> Result<bool, FromUtf8Error> {
        let output = exec_command(cmd).unwrap();

        match expected {
            Some(exp) => {
                let out_str = String::from_utf8(output.stdout)?;
                if out_str == exp {
                    return Ok(true);
                }
                return Ok(false);
            }
            None => {
                let out_str = String::from_utf8(output.stderr)?;
                if out_str.len() == 0 {
                    return Ok(true);
                }
                return Ok(false);
            }
        }
    }
}

struct ProgramManager<'a> {
    programs: Vec<Program<'a>>,
}

impl<'a> ProgramManager<'a> {
    fn new() -> ProgramManager<'a> {
        return ProgramManager {
            programs: Vec::<Program>::new(),
        };
    }

    fn add_program(&mut self, prog: Program<'a>) {
        self.programs.push(prog);
    }

    fn start_programs(&mut self) {
        for program in self.programs.iter() {
            program.start();
        }
    }

    fn check_programs(&mut self) {
        for program in self.programs.iter() {
            program.check();
        }
    }
}

fn exec_command(cmd_string: &str) -> Result<Output, CommandParseError> {
    let cmds: Vec<&str> = cmd_string.split_whitespace().collect();
    if cmds.len() == 0 {
        return Err(CommandParseError {});
    }
    let mut cmd = &mut Command::new(cmds[0]);
    for i in 1..cmds.len() - 1 {
        cmd = cmd.arg(cmds[i]);
    }
    let output = cmd.stdout(Stdio::piped()).output().unwrap();
    return Ok(output);
}

fn main() {
    let mut manager = ProgramManager::new();

    let prog = Program::new("Test".to_owned());
    manager.add_program(prog);
    manager.start_programs();
    manager.check_programs();
}
