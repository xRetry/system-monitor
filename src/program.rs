use std::error::Error;
use std::process::{Command, Output, Stdio};
use std::string::FromUtf8Error;

#[derive(Debug)]
struct ParsingError {}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error parsing command")
    }
}

impl Error for ParsingError {}

impl ParsingError {
    fn new() -> Box<ParsingError> {
        Box::new(ParsingError {})
    }
}

pub struct Program<'a> {
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

    pub fn check(&self) -> Result<bool, Box<dyn Error>> {
        let is_ok = self.compare_to_expected(self.cmd_status, self.expected_status)?;
        return Ok(is_ok);
    }

    pub fn start(&self) -> Result<bool, Box<dyn Error>> {
        let is_ok = self.compare_to_expected(self.cmd_start, self.expected_start)?;
        return Ok(is_ok);
    }

    fn compare_to_expected(
        &self,
        cmd: &str,
        expected: Option<&str>,
    ) -> Result<bool, Box<dyn Error>> {
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

fn exec_command(cmd_string: &str) -> Result<Output, Box<dyn Error>> {
    let cmds: Vec<&str> = cmd_string.split_whitespace().collect();
    if cmds.len() == 0 {
        return Err(ParsingError::new());
    }
    let mut cmd = &mut Command::new(cmds[0]);
    for i in 1..cmds.len() - 1 {
        cmd = cmd.arg(cmds[i]);
    }
    let output = cmd.stdout(Stdio::piped()).output().unwrap();
    return Ok(output);
}
