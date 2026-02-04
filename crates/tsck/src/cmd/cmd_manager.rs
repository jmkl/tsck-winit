use anyhow::{Context, Result};
use flume::{Receiver, Sender, unbounded};
use parking_lot::Mutex;
use shared_child::SharedChild;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    os::windows::process::CommandExt,
    process::{Command, Stdio},
    sync::Arc,
    thread,
    time::Duration,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Stdout,
    Stderr,
}

#[derive(Debug, Clone)]
pub struct ProcessOutput {
    pub pid: u32,
    pub name: Arc<str>,
    pub level: LogLevel,
    pub message: Arc<str>,
}

impl ProcessOutput {
    pub fn print_colored(&self) {
        let color = match self.level {
            LogLevel::Stdout => 36, // Cyan
            LogLevel::Stderr => 31, // Red
        };
        let level_str = match self.level {
            LogLevel::Stdout => "OUT",
            LogLevel::Stderr => "ERR",
        };
        eprintln!(
            "\x1b[{}m[{}:{}:{}]\x1b[0m {}",
            color, self.name, self.pid, level_str, self.message
        );
    }
}

// ============================================================================
// Process Manager
// ============================================================================

pub struct ProcessManager {
    processes: Arc<Mutex<HashMap<u32, Arc<ManagedProcess>>>>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            processes: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn is_running(&self, pid: i32) -> bool {
        if pid == 0 {
            return false;
        }

        Command::new("tasklist")
            .creation_flags(0x08000000)
            .args(["/FI", &format!("PID eq {}", pid)])
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).contains(&pid.to_string()))
            .unwrap_or(false)
    }

    pub fn spawn(
        &self,
        app_name: impl AsRef<str>,
        program: impl AsRef<str>,
        args: &[&str],
        work_dir: &str,
    ) -> Result<Arc<ManagedProcess>> {
        let app_name = app_name.as_ref();
        let program = program.as_ref();
        let process = ManagedProcess::new(app_name, program, args, work_dir)?;
        let pid = process.pid();
        let process = Arc::new(process);

        self.processes.lock().insert(pid, process.clone());
        Ok(process)
    }

    pub fn get(&self, pid: u32) -> Option<Arc<ManagedProcess>> {
        self.processes.lock().get(&pid).cloned()
    }

    //	FIXME! need this, just in case the the setting pid
    //  not set to `0` in config file
    fn task_kill(&self, pid: u32) {
        let _ = Command::new("taskkill")
            .args(["/F", "/PID", &pid.to_string(), "/T"])
            .creation_flags(0x08000000)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .creation_flags(0x08000000)
            .spawn();
    }
    pub fn kill_app(&self, app_name: &str) -> Result<()> {
        if let Some(result) = self
            .processes
            .lock()
            .iter()
            .find(|(_, mp)| mp.app_name.contains(app_name))
        {
            result.1.kill()?;
            self.task_kill(result.1.pid);
        }
        Ok(())
    }

    pub fn list(&self) -> Vec<u32> {
        self.processes.lock().keys().copied().collect()
    }

    pub fn kill(&self, pid: u32) -> Result<()> {
        let process = self.get(pid).context("Process not found")?;
        process.kill()?;
        self.processes.lock().remove(&pid);
        Ok(())
    }

    pub fn kill_all(&self) -> Result<()> {
        let procs = self.processes.lock();
        for process in procs.values() {
            let _ = process.kill();
        }
        drop(procs);
        self.processes.lock().clear();
        Ok(())
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ProcessManager {
    fn drop(&mut self) {
        let _ = self.kill_all();
    }
}

// ============================================================================
// Single Process
// ============================================================================

pub struct ManagedProcess {
    pid: u32,
    app_name: Arc<str>,
    program: Arc<str>,
    child: Arc<SharedChild>,
    receiver: Receiver<ProcessOutput>,
}

impl ManagedProcess {
    fn new(app_name: &str, program: &str, args: &[&str], work_dir: &str) -> Result<Self> {
        let mut cmd = Command::new(program);
        cmd.creation_flags(0x08000000)
            .current_dir(work_dir)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let child = SharedChild::spawn(&mut cmd)?;
        let pid = child.id();
        let name: Arc<str> = Arc::from(app_name);

        let (sender, receiver) = unbounded();
        let stdout = child.take_stdout().context("Failed to capture stdout")?;
        let stderr = child.take_stderr().context("Failed to capture stderr")?;

        // Spawn stdout reader thread
        {
            let sender = sender.clone();
            let name = name.clone();
            thread::spawn(move || {
                let reader = BufReader::new(stdout);
                Self::read_stream(reader, sender, name, pid, LogLevel::Stdout);
            });
        }

        // Spawn stderr reader thread
        {
            let name = name.clone();
            thread::spawn(move || {
                let reader = BufReader::new(stderr);
                Self::read_stream(reader, sender, name, pid, LogLevel::Stderr);
            });
        }

        Ok(ManagedProcess {
            app_name: Arc::from(app_name),
            pid,
            program: name,
            child: Arc::new(child),
            receiver,
        })
    }

    fn read_stream<R: BufRead + Send + 'static>(
        reader: R,
        sender: Sender<ProcessOutput>,
        name: Arc<str>,
        pid: u32,
        level: LogLevel,
    ) {
        let reader = BufReader::new(reader);
        for line in reader.lines().flatten() {
            let output = ProcessOutput {
                pid,
                name: name.clone(),
                level,
                message: Arc::from(line),
            };
            let _ = sender.send(output);
        }
    }

    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn name(&self) -> &str {
        &self.program
    }

    pub fn is_running(&self) -> bool {
        self.child.try_wait().ok().flatten().is_none()
    }

    pub fn kill(&self) -> Result<()> {
        self.child.kill().context("Failed to kill process")
    }

    pub fn try_recv(&self) -> Result<ProcessOutput> {
        self.receiver.try_recv().context("No output available")
    }

    pub fn recv(&self) -> Result<ProcessOutput> {
        self.receiver.recv().context("Channel closed")
    }

    pub fn recv_timeout(&self, timeout: Duration) -> Result<ProcessOutput> {
        self.receiver
            .recv_timeout(timeout)
            .context("Timeout waiting for output")
    }

    pub fn iter(&self) -> impl Iterator<Item = ProcessOutput> + '_ {
        self.receiver.iter()
    }
}

// ============================================================================
// Builder API for easier spawning
// ============================================================================

pub struct ProcessBuilder {
    app_name: String,
    program: String,
    args: Vec<String>,
    work_dir: String,
}

impl ProcessBuilder {
    pub fn new(
        app_name: impl Into<String>,
        program: impl Into<String>,
        work_dir: impl Into<String>,
    ) -> Self {
        Self {
            program: program.into(),
            app_name: app_name.into(),
            args: Vec::new(),
            work_dir: work_dir.into(),
        }
    }

    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.args.extend(args.into_iter().map(|s| s.into()));
        self
    }

    pub fn spawn_with(self, manager: &ProcessManager) -> Result<Arc<ManagedProcess>> {
        let arg_strs: Vec<&str> = self.args.iter().map(|s| s.as_str()).collect();
        manager.spawn(&self.app_name, &self.program, &arg_strs, &self.work_dir)
    }
}

// ============================================================================
// Convenience Functions
// ============================================================================

pub fn run_command(
    app_name: &str,
    program: &str,
    args: &[&str],
    work_dir: &str,
    mut callback: impl FnMut(ProcessOutput),
) -> Result<()> {
    let manager = ProcessManager::new();
    let process = manager.spawn(app_name, program, args, work_dir)?;

    while process.is_running() {
        if let Ok(output) = process.try_recv() {
            callback(output);
        }
        thread::sleep(Duration::from_millis(10));
    }

    // Drain remaining output
    while let Ok(output) = process.try_recv() {
        callback(output);
    }

    Ok(())
}
