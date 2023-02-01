use crate::errors::Error;
use serde::Deserialize;
use std::process::{Command, Output, Stdio};

#[derive(Clone, Copy, Deserialize, Debug)]
enum Status {
    Running,
    Stopped,
    Problem,
}

enum Action {
    Start,
    Stop,
}

#[derive(Debug, Deserialize)]
pub struct Program {
    name: String,
    status: Option<Status>,
    status_target: Option<Status>,
    cmd_start: Option<String>,
    expected_start: Option<String>,
    cmd_status: String,
    expected_status: Option<String>,
    cmd_stop: Option<String>,
    expected_stop: Option<String>,
}

impl Program {
    pub fn new(name: String) -> Program {
        return Program {
            name: name,
            status: None,
            status_target: None,
            cmd_start: Some("ls -la".to_owned()),
            expected_start: None,
            cmd_status: "echo hello".to_owned(),
            expected_status: Some("hello".to_owned()),
            cmd_stop: Some("ls -la".to_owned()),
            expected_stop: Some("hello".to_owned()),
        };
    }

    pub fn check(&mut self, act_on_result: bool) {
        let status = match self.exec_and_compare(&self.cmd_status, &self.expected_status) {
            Ok(s) => match s {
                true => Status::Running,
                false => Status::Stopped,
            },
            Err(_) => Status::Problem,
        };

        self.status = Some(status);

        if !act_on_result {
            return;
        }

        let mut next_action = None;
        match self.status_target {
            None => (),
            Some(stat_tar) => match status {
                Status::Problem => next_action = Some(Action::Start),
                Status::Stopped => match stat_tar {
                    Status::Running => next_action = Some(Action::Start),
                    _ => (),
                },
                Status::Running => match stat_tar {
                    Status::Stopped => next_action = Some(Action::Stop),
                    _ => (),
                },
            },
        }

        match next_action {
            None => (),
            Some(Action::Start) => self.start(),
            Some(Action::Stop) => self.stop(),
        }
    }

    pub fn start(&mut self) {
        match &self.cmd_start {
            None => (),
            Some(cmd) => {
                let _ = self.exec_and_compare(&cmd, &self.expected_start);
                self.check(false);
            }
        }
    }

    pub fn stop(&mut self) {
        match &self.cmd_stop {
            None => (),
            Some(cmd) => {
                let _ = self.exec_and_compare(&cmd, &self.expected_stop);
                self.check(false);
            }
        }
    }

    fn exec_and_compare(&self, cmd: &String, expected: &Option<String>) -> Result<bool, Error> {
        let output = exec_command(cmd)?;

        match expected {
            Some(exp) => {
                let out_str = String::from_utf8(output.stdout)?;
                if out_str == *exp {
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

fn exec_command(cmd_string: &String) -> Result<Output, Error> {
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
