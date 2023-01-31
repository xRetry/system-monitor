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
    expected_start: Option<&'a str>,
    cmd_status: &'a str,
    expected_status: Option<&'a str>,
    cmd_stop: &'a str,
    expected_stop: Option<&'a str>,
}

impl<'a> Program<'a> {
    pub fn new(name: String) -> Program<'a> {
        return Program {
            name: name,
            status: Status::Unknown,
            status_target: Status::Unknown,
            cmd_start: "ls -la",
            expected_start: None,
            cmd_status: "echo hello",
            expected_status: Some("hello"),
            cmd_stop: "ls -la",
            expected_stop: Some("hello"),
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

        match status {
            Status::Problem => self.start(),
            Status::Disabled => match self.status_target {
                Status::Running | Status::Stopped => self.start(),
                _ => (),
            },
            Status::Running => match self.status_target {
                Status::Disabled => self.stop(),
                _ => (),
            },
            _ => (),
        }
    }

    pub fn start(&self) {
        let _ = self.compare_to_expected(self.cmd_start, self.expected_start);
        // TODO: Check Status without recursion
    }

    pub fn stop(&self) {
        let _ = self.compare_to_expected(self.cmd_stop, self.expected_stop);
        // TODO: Check Status without recursion
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
