use crate::errors::Error;
use std::process::{Command, Output, Stdio};

enum Status {
    Running,
    Stopped,
    Problem,
    Disabled,
    Unknown,
}

pub struct Program<'a> {
    name: String,
    status: Status,
    status_target: Status,
    cmd_start: &'a str,
    cmd_status: &'a str,
    expected_start: Option<&'a str>,
    expected_status: Option<&'a str>,
}

impl<'a> Program<'a> {
    pub fn new(name: String) -> Program<'a> {
        return Program {
            name: name,
            status: Status::Unknown,
            status_target: Status::Unknown,
            cmd_start: "ls -la",
            cmd_status: "echo hello",
            expected_start: None,
            expected_status: Some("hello"),
        };
    }

    pub fn check(&self) {
        let status = match self.compare_to_expected(self.cmd_status, self.expected_status) {
            Ok(s) => match s {
                true => Status::Running,
                false => Status::Stopped,
            },
            Err(_) => Status::Problem,
        };

        // TODO: Handle status cases
        todo!();
    }

    pub fn start(&self) -> Result<bool, Error> {
        let is_ok = self.compare_to_expected(self.cmd_start, self.expected_start)?;
        return Ok(is_ok);
    }

    fn compare_to_expected(&self, cmd: &str, expected: Option<&str>) -> Result<bool, Error> {
        let output = exec_command(cmd)?;

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

fn exec_command(cmd_string: &str) -> Result<Output, Error> {
    let cmds: Vec<&str> = cmd_string.split_whitespace().collect();
    if cmds.len() == 0 {
        return Err(Error::ParsingError);
    }
    let mut cmd = &mut Command::new(cmds[0]);
    for i in 1..cmds.len() - 1 {
        cmd = cmd.arg(cmds[i]);
    }
    let output = cmd.stdout(Stdio::piped()).output().unwrap();
    return Ok(output);
}
