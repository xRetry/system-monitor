use std::process::{Command, Output, Stdio};

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

    pub fn check(&self) {
        self.compare_to_expected(self.cmd_status, self.expected_status);
    }

    pub fn start(&self) {
        self.compare_to_expected(self.cmd_start, self.expected_start);
    }

    fn compare_to_expected(&self, cmd: &str, expected: Option<&str>) -> bool {
        let output = exec_command(cmd).unwrap();

        match expected {
            Some(exp) => {
                let out_str = String::from_utf8(output.stdout).unwrap();
                if out_str == exp {
                    return true;
                }
                return false;
            }
            None => {
                let out_str = String::from_utf8(output.stderr).unwrap();
                if out_str.len() == 0 {
                    return true;
                }
                return false;
            }
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
    let prog = Program::new("Test".to_owned());
    prog.check();
}
