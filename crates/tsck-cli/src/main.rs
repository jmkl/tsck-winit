use anyhow::Result;

use flume::{Sender, unbounded};
mod config;
use std::{
    io::{self, BufRead, BufReader},
    process::{Command, Stdio},
};
struct CmdHelper {
    exe: String,
    args: Vec<String>,
    work_dir: String,
    sender: Sender<(String, String)>,
}
impl CmdHelper {
    fn new(
        exe: &str,
        args: &Vec<String>,
        work_dir: &str,
        sender: Sender<(String, String)>,
    ) -> Self {
        Self {
            exe: exe.to_string(),
            args: args.iter().map(|f| f.to_string()).collect(),
            work_dir: work_dir.to_string(),
            sender,
        }
    }
    fn run(&self) -> Result<()> {
        let exe = self.exe.clone();
        let args = self.args.clone();
        let work_dir = self.work_dir.clone();
        let sender = self.sender.clone();
        std::thread::spawn(move || -> Result<()> {
            let mut cmd = Command::new(exe.clone())
                .args(args)
                .current_dir(work_dir)
                .stdout(Stdio::piped())
                .stdin(Stdio::piped())
                .spawn()?;
            if let Some(stdout) = cmd.stdout.take() {
                let reader = BufReader::new(stdout);
                for line in reader.lines().map_while(Result::ok) {
                    sender.send((exe.clone(), line))?;
                }
            }
            if let Some(stderr) = cmd.stderr.take() {
                let reader = BufReader::new(stderr);
                for line in reader.lines().map_while(Result::ok) {
                    sender.send((exe.clone(), line))?;
                }
            }
            Ok(())
        });
        Ok(())
    }
}

#[derive(Debug)]
enum Mode {
    Run,
    None,
}

fn printn_help() {
    println!(
        r#"
--------------------------
run					: run command
--------------------------"#
    );
}

fn run_it_down() -> Result<()> {
    let root = std::env::current_dir()?;
    let root = root.to_str().unwrap_or_else(|| "");
    let (tx, rx) = unbounded();

    if let Ok(config) = crate::config::read_config() {
        for cmd in config.commands.iter() {
            CmdHelper::new(&cmd.exe, &cmd.args, root, tx.clone()).run()?;
        }
    }
    while let Ok((exe, log)) = rx.recv() {
        println!("[{}] {}", exe, log);
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let mode = match args.next().as_deref() {
        Some("run") => Mode::Run,
        _ => Mode::None,
    };
    match mode {
        Mode::Run => run_it_down()?,
        Mode::None => printn_help(),
    }
    println!("Hello");
    Ok(())
}
