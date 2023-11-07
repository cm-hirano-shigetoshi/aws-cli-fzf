use std::io::{Error, Result};
use std::path::Path;
use std::process::{Command, Stdio};

use std::fs::OpenOptions;
use std::io::prelude::*;
pub fn debug(s: &str) {
    //let mut file = File::create("/tmp/aaa").unwrap();
    let mut file = OpenOptions::new().append(true).open("/tmp/aaa").unwrap();
    file.write_all(format!("{}\n", s).as_bytes()).unwrap();
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
    let new_buffer =
        make_new_buffer_from_file(tool_dir, help_dir, command.as_str(), options.as_str());
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
    let path = get_command_file_path(help_dir, command);
    if !Path::new(path.as_str()).exists() {
        execute_command(
            format!(
                "mkdir -p $(dirname {}) && aws {} help | fzf --ansi -f ^ > {}",
                path,
                command.to_string().replace(":", " "),
                path
            )
            .as_str(),
        )
        .unwrap_or_else(|_err| String::from(""));
    }
    let fzf_command = format!(
        "bash {}/bash/option_list.sh '{}' | fzf --reverse --ansi",
        tool_dir, path
    );
    return execute_command(fzf_command.as_str())
        .unwrap_or_else(|_err| String::from(""))
        .trim_end()
        .to_string();
}

pub fn get_command_file_path(help_dir: &str, command: &str) -> String {
    return format!("{}/commands/{}", help_dir, command.replace(":", "/"));
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

pub fn make_new_buffer_from_file(
    tool_dir: &str,
    help_dir: &str,
    command: &str,
    options: &str,
) -> String {
    let path = get_command_file_path(help_dir, command);
    let content = execute_command(
        format!(
            "bash {}/bash/option_list.sh '{}' | fzf --ansi -f ^",
            tool_dir, path
        )
        .as_str(),
    )
    .unwrap_or_else(|_err| String::from(""));
    return make_new_buffer_from_str(command, content.trim_end(), options);
}

pub fn make_new_buffer_from_str(command: &str, content: &str, options: &str) -> String {
    let content_vec: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
    let options_vec: Vec<String> = options.split("\n").map(|s| s.to_string()).collect();
    return make_new_buffer(command, content_vec, options_vec);
}

pub fn make_new_buffer(command: &str, lines: Vec<String>, options: Vec<String>) -> String {
    return format!(
        "aws {} {} {}",
        command.replace(":", " "),
        get_mondatories(lines),
        get_options(options)
    );
}

pub fn get_mondatories(lines: Vec<String>) -> String {
    let mut result = String::new();
    for line in lines {
        if !line.starts_with("[") {
            result.push_str(line.as_str());
            result.push(' ');
        }
    }
    if result.len() > 0 {
        result.pop();
    }
    return result;
}

pub fn get_options(options: Vec<String>) -> String {
    let mut result = String::new();
    for option in options {
        if option.starts_with("[") {
            result.push_str(&option[1..option.len() - 1]);
            result.push(' ');
        }
    }
    if result.len() > 0 {
        result.pop();
    }
    return result;
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
        let lines: Vec<String> = vec![
            "--function-name <value>".to_string(),
            "<outfile>".to_string(),
            "[--endpoint-url <value>]".to_string(),
        ];
        let options: Vec<String> = vec!["[--endpoint-url <value>]".to_string()];
        let result = make_new_buffer("lambda:invoke", lines, options);
        assert_eq!(
            result,
            "aws lambda invoke --function-name <value> <outfile> --endpoint-url <value>"
        );
    }

    #[test]
    fn test_get_mondatories() {
        let lines: Vec<String> = vec![
            "--function-name <value>".to_string(),
            "<outfile>".to_string(),
            "[--endpoint-url <value>]".to_string(),
        ];
        let result = get_mondatories(lines);
        assert_eq!(result, "--function-name <value> <outfile>");

        let lines: Vec<String> = vec![
            "[--endpoint-url <value>]".to_string(),
            "[--hogehoge <value>]".to_string(),
        ];
        let result = get_mondatories(lines);
        assert_eq!(result, "");
    }

    #[test]
    fn test_get_options() {
        let options: Vec<String> = vec!["[--endpoint-url <value>]".to_string()];
        let result = get_options(options);
        assert_eq!(result, "--endpoint-url <value>");

        let options: Vec<String> = vec!["--endpoint-url <value>".to_string()];
        let result = get_options(options);
        assert_eq!(result, "");
    }
}
