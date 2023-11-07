use std::io::BufReader;
use std::io::{Error, Result};
use std::path::Path;
use std::process::{Command, Stdio};

use std::fs::File;
use std::io::prelude::*;
pub fn debug(s: &str) -> std::io::Result<()> {
    let mut file = File::create("/tmp/aaa")?;
    file.write_all(s.as_bytes())?;
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Please provide one argument");
        return;
    }
    let help_dir: &str = &args[1];
    let tool_dir: &str = &args[2];
    let command = execute_fzf_for_command(help_dir, tool_dir);
    let options = execute_fzf_for_options(help_dir, tool_dir, command.as_str());
    let new_buffer = make_new_buffer_from_file(help_dir, command.as_str(), options.as_str());
    if new_buffer.len() > 0 {
        let new_cursor = new_buffer.len();
        println!("{} {}", new_cursor, new_buffer);
    }
}

pub fn execute_fzf_for_command(help_dir: &str, tool_dir: &str) -> String {
    let fzf_command = format!(
        "bash {}/bash/command_list.sh {}/services | fzf --reverse --ansi",
        tool_dir, help_dir
    );
    return execute_command(fzf_command.as_str())
        .unwrap_or_else(|_err| String::from(""))
        .trim_end()
        .to_string();
}

pub fn execute_fzf_for_options(help_dir: &str, tool_dir: &str, command: &str) -> String {
    let path = format!("{}/commands/{}", help_dir, command.replace(":", "/"));
    if !Path::new(path.as_str()).exists() {
        execute_command(
            format!(
                "aws {} help | fzf --ansi -f ^ > {}",
                command.to_string().replace(":", " "),
                path
            )
            .as_str(),
        )
        .unwrap_or_else(|_err| String::from(""));
    }
    let fzf_command = format!(
        "bash {}/bash/option_list.sh '{}/commands' '{}' | fzf --reverse --ansi",
        tool_dir, help_dir, command
    );
    return execute_command(fzf_command.as_str()).unwrap_or_else(|_err| String::from(""));
}

pub fn execute_command(command: &str) -> Result<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stderr(Stdio::inherit())
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout).unwrap())
    } else {
        Err(Error::new(
            std::io::ErrorKind::Other,
            "Command execution failed",
        ))
    }
}

pub fn make_new_buffer_from_file(help_dir: &str, command: &str, options: &str) -> String {
    let path = format!("{}/commands/{}", help_dir, command.replace(":", "/"));
    let lines = read_file(path.as_str());
    return make_new_buffer(lines, options.split("\n").map(|s| s.to_string()).collect());
}

pub fn read_file(path: &str) -> Vec<String> {
    let f = File::open(path).expect("file not found");
    let br = BufReader::new(f);
    let mut lines: Vec<String> = Vec::new();
    for l in br.lines() {
        let line: String = l.expect("fail to read line");
        lines.push(line);
    }
    return lines;
}

pub fn make_new_buffer(lines: Vec<String>, options: Vec<String>) -> String {
    return options[0].clone();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_command() {
        let result = execute_command("echo aaa").unwrap();
        assert_eq!(result, "aaa\n");
    }

    #[test]
    fn test_make_new_buffer() {
        let lines: Vec<String> = vec!["".to_string()];
        let options: Vec<String> = vec!["aaa".to_string()];
        let result = make_new_buffer(lines, options);
        assert_eq!(result, "aaa");
    }
}
