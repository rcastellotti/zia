use crate::config;
use chrono::{DateTime, Utc};
use colored::*;
use serde::Serialize;
use similar::{ChangeTag, TextDiff};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug, Serialize)]
pub struct Report {
    timestamp: DateTime<Utc>,
    command_comparisons: Vec<CommandComparison>,
}

impl PartialEq for Report {
    fn eq(&self, other: &Self) -> bool {
        if self.command_comparisons.len() != other.command_comparisons.len() {
            return false;
        }
        self.command_comparisons
            .iter()
            .zip(other.command_comparisons.iter())
            .all(|(a, b)| a == b)
    }
}

impl Report {
    pub fn generate_summary(&self) -> String {
        let mut summary = format!(
            "{} ({} commands)\n\n",
            "Zia report summary".white().bold().underline(),
            self.command_comparisons.len()
        );

        for i in &self.command_comparisons {
            let status = if i.ce1.stdout == i.ce2.stdout
                && i.ce1.stderr == i.ce2.stderr
                && i.ce1.retcode == i.ce2.retcode
            {
                "ok".green()
            } else {
                "FAILED".red()
            };
            summary.push_str(&format!("+ cmd {:?} ... {}\n", i.name, status));
        }

        summary
    }

    pub fn print_summary(&self) {
        println!("{}", self.generate_summary());
    }

    pub fn print_json(&self) {
        let json_data = serde_json::to_string_pretty(&self)
            .expect("Something went wrong serializing the report");

        println!("{}", json_data);
    }
}

#[derive(Debug, PartialEq, Serialize)]
struct CommandComparison {
    name: String,
    diff_stdout: Option<Vec<String>>,
    diff_stderr: Option<Vec<String>>,
    ce1: CommandExecution,
    ce2: CommandExecution,
}
#[derive(Debug, Serialize)]
pub struct CommandExecution {
    stdout: String,
    stderr: String,
    retcode: i32,
    duration: u128,
    log_file: Option<String>,
    env: Option<HashMap<String, String>>,
    binary_realpath: PathBuf,
}

impl PartialEq for CommandExecution {
    fn eq(&self, other: &Self) -> bool {
        self.stdout == other.stdout
            && self.stderr == other.stderr
            && self.retcode == other.retcode
            && self.log_file == other.log_file
            && self.env == other.env
            && self.binary_realpath == other.binary_realpath
    }
}

fn enrich_diff_htmlf(fd1: &str, fd2: &str) -> Option<Vec<String>> {
    if fd1 == fd2 {
        return None;
    }

    let diff: TextDiff<'_, '_, '_, str> = TextDiff::from_lines(fd1, fd2);

    let res = diff
        .iter_all_changes()
        .map(|change| {
            let prefix = match change.tag() {
                ChangeTag::Delete => "<span class='text-danger'>",
                ChangeTag::Insert => "<span class='text-success'>",
                ChangeTag::Equal => "<span>",
            };
            format!("{}{}{}", prefix, change.value().trim_end(), "</span>")
        })
        .collect();
    Some(res)
}

pub fn run(config: &config::Config) -> Report {
    let mut rep = Report {
        timestamp: chrono::offset::Utc::now(),
        command_comparisons: Vec::new(),
    };
    for c in &config.commands {
        let ce1 = run_command(&config.bin1, &c);
        let ce2 = run_command(&config.bin2, &c);

        let res: CommandComparison = CommandComparison {
            name: c.name.clone(),
            diff_stdout: enrich_diff_htmlf(&ce1.stdout, &ce2.stdout),
            diff_stderr: enrich_diff_htmlf(&ce1.stderr, &ce2.stderr),
            ce1,
            ce2,
        };
        rep.command_comparisons.push(res);
    }
    rep
}

fn run_command(bin: &PathBuf, cmd: &config::Command) -> CommandExecution {
    let start = Instant::now();
    log::debug!(
        "Running command: {:#?} {:}",
        bin.clone(),
        cmd.args.join(" ")
    );
    let out = std::process::Command::new(bin)
        .args(&cmd.args)
        .output()
        .expect("Failed running a command");
    let duration = start.elapsed().as_millis();

    CommandExecution {
        stdout: String::from_utf8(out.stdout).expect("Stdout is always UTF-8"),
        stderr: String::from_utf8(out.stderr).expect("Stderr is always UTF-8"),
        retcode: out.status.code().unwrap(),
        duration,
        log_file: None,
        env: cmd.env.clone(),
        binary_realpath: bin.to_path_buf(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;

    #[test]
    fn test_print_summary() {
        let camd: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        struct TestCase {
            name: String,
            want: String,
            report: Report,
        }

        let test_cases = vec![
            TestCase {
                name: String::from("basic report works"),
                want: format!(
                    "{} (1 commands)\n\n+ cmd {:?} ... {}\n",
                    "Zia report summary".white().bold().underline(),
                    "test_cmd",
                    "ok".green()
                ),
                report: Report {
                    timestamp: chrono::offset::Utc::now(),
                    command_comparisons: vec![CommandComparison {
                        name: "test_cmd".to_string(),
                        diff_stderr: None,
                        diff_stdout: None,

                        ce1: CommandExecution {
                            stdout: "Added 'Giorgio' to the zoo.
                            Animals in the zoo:
                            Gator the The Alligator
                            Giorgio the Scuttlebuff
                            Manny the The Monkey
                            Melania the The Cute Bear
                            "
                            .to_string(),
                            stderr: "".to_string(),
                            retcode: 0,
                            duration: 4_980_375,
                            log_file: None,
                            env: Some(HashMap::from([("ZIA_ENV1".to_string(), "r".to_string())])),
                            binary_realpath: "/Users/rob/zia/target/debug/zoo".into(),
                        },
                        ce2: CommandExecution {
                            stdout: "Added 'Giorgio' to the zoo.
                            Animals in the zoo:
                            Gator the The Alligator
                            Giorgio the Scuttlebuff
                            Manny the The Monkey
                            Melania the The Cute Bear
                            "
                            .to_string(),
                            stderr: "".to_string(),
                            retcode: 0,
                            duration: 8_992_125,
                            log_file: None,
                            env: Some(HashMap::from([("ZIA_ENV1".to_string(), "r".to_string())])),
                            binary_realpath: camd.join("target").join("debug").join("zoo"),
                        },
                    }],
                },
            },
            TestCase {
                name: "one command ok".to_string(),
                want: format!(
                    "{} (1 commands)\n\n+ cmd {:?} ... {}\n",
                    "Zia report summary".white().bold().underline(),
                    "test_cmd",
                    "ok".green()
                ),
                report: Report {
                    timestamp: chrono::offset::Utc::now(),
                    command_comparisons: vec![CommandComparison {
                        name: "test_cmd".to_string(),
                        diff_stderr: None,
                        diff_stdout: None,
                        ce1: CommandExecution {
                            stdout: "out".to_string(),
                            stderr: "err".to_string(),
                            retcode: 0,
                            duration: 1,
                            log_file: None,
                            env: Some(HashMap::from([("ZIA_ENV1".to_string(), "r".to_string())])),
                            binary_realpath: PathBuf::from("/"),
                        },
                        ce2: CommandExecution {
                            stdout: "out".to_string(),
                            stderr: "err".to_string(),
                            retcode: 0,
                            duration: 1,
                            log_file: None,
                            env: Some(HashMap::from([("ZIA_ENV1".to_string(), "r".to_string())])),
                            binary_realpath: PathBuf::from("/"),
                        },
                    }],
                },
            },
            TestCase {
                name: "one command different retcodes".to_string(),
                want: format!(
                    "{} (1 commands)\n\n+ cmd {:?} ... {}\n",
                    "Zia report summary".white().bold().underline(),
                    "test_cmd",
                    "FAILED".red()
                ),
                report: Report {
                    timestamp: chrono::offset::Utc::now(),
                    command_comparisons: vec![CommandComparison {
                        name: "test_cmd".to_string(),
                        diff_stderr: None,
                        diff_stdout: None,
                        ce1: CommandExecution {
                            stdout: "out".to_string(),
                            stderr: "err".to_string(),
                            retcode: 1,
                            duration: 1,
                            log_file: None,
                            env: Some(HashMap::from([("ZIA_ENV1".to_string(), "r".to_string())])),
                            binary_realpath: PathBuf::from("/"),
                        },
                        ce2: CommandExecution {
                            stdout: "out".to_string(),
                            stderr: "err".to_string(),
                            retcode: 0,
                            duration: 1,
                            log_file: None,
                            env: Some(HashMap::from([("ZIA_ENV1".to_string(), "r".to_string())])),
                            binary_realpath: PathBuf::from("/"),
                        },
                    }],
                },
            },
        ];

        for tc in test_cases {
            assert_eq!(
                tc.want,
                tc.report.generate_summary(),
                "Error generating summary: \n{}",
                tc.name
            );
        }
    }
}
