pub mod cmd_manager;
use crate::app_config::AppConfigHandler;
use crate::event::{ChannelEvent, EventPayload, UserEvent};
use crate::ipc::{IpcHelper, IpcResponse};
use crate::{ChannelBus, app, dp, log_debug, log_error, ts_struct};
use crate::{cmd::cmd_manager::ProcessManager, event::TS_PATH};
use flume::Sender;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::sync::Arc;
use std::thread::{self, sleep};
use std::time::Duration;
use ts_rs::TS;
use winit::event_loop::EventLoopProxy;
pub const COMMAND_CONFIG_KEY: &str = "command_config";

ts_struct! {path=TS_PATH,
    pub struct AppCommand {
        pub name: String,
        #[serde(default)]
        pub pid: i32,
        pub executable: Option<String>,
        pub work_dir: Option<String>,
        pub args: Vec<String>,
        #[serde(default)]
        pub logs: Vec<String>
    }
}

ts_struct! {path=TS_PATH,
    pub enum LogType {
        Pid,
        Stdout,
        Stderr
    }
}
ts_struct! {path=TS_PATH,
    pub struct CmdrLog {
        pub log_type: LogType,
        pub app_name: String,
        pub content: Option<String>,
        pub pid: Option<i32>,
    }
}
impl CmdrLog {
    pub fn pid(app_name: impl Into<String>, pid: u32) -> Self {
        CmdrLog {
            log_type: LogType::Pid,
            app_name: app_name.into(),
            content: None,
            pid: Some(pid as i32),
        }
    }
    pub fn stdout(
        log_type: LogType,
        app_name: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        CmdrLog {
            log_type,
            app_name: app_name.into(),
            content: Some(content.into()),
            pid: None,
        }
    }
}

ts_struct! {path=TS_PATH,
    pub struct CommandConfig {
        pub log_limit: i32,
        pub commands: Vec<AppCommand>,
    }
}

impl Default for CommandConfig {
    fn default() -> Self {
        Self {
            log_limit: 20,
            commands: Vec::new(),
        }
    }
}

pub struct CmdrHelper {
    process_manager: Arc<ProcessManager>,
    app_config: Arc<Mutex<AppConfigHandler>>,
    proxy: Arc<EventLoopProxy>,
}

macro_rules! cmd_sender {
    () => {};
}

impl CmdrHelper {
    pub fn new(config: Arc<Mutex<AppConfigHandler>>, proxy: Arc<EventLoopProxy>) -> Self {
        let process_manager = Arc::new(ProcessManager::new());

        Self {
            process_manager: process_manager,
            app_config: config,
            proxy,
        }
    }
    pub fn check_pids(&self) {
        let mut guard = self.app_config.lock();
        {
            let mut store = guard.config_store.get(|f| f.clone());
            for f in store.command_config.commands.iter_mut() {
                if !self.process_manager.is_running(f.pid) {
                    f.pid = 0;
                }
            }
            guard.update_config(store);
        }
    }
    fn update_pid(pid: u32, app_name: &str, cmd_config: Arc<Mutex<AppConfigHandler>>) {
        let mut guard = cmd_config.lock();
        {
            let mut store = guard.config_store.get(|f| f.clone());
            for f in store.command_config.commands.iter_mut() {
                if f.name == app_name {
                    log_debug!("UPDATE PID ", &pid);
                    f.pid = pid as i32;
                }
            }
            guard.update_config(store);
        }
    }

    pub fn run_command(
        &self,
        app_name: String,
        sender: Sender<ChannelEvent>,
    ) -> anyhow::Result<()> {
        let commands = {
            let commands = self.app_config.lock().command_config().commands;
            commands
        };
        commands.iter().find(|c| c.name == app_name).map(|c| {
            let args: Vec<String> = c.args.clone();
            let work_dir = c.work_dir.clone().unwrap_or("/".to_string());
            let app_name = app_name.clone();
            let exe = c.executable.clone();
            let sender = sender.clone();
            let app_config = self.app_config.clone();
            let proxy = self.proxy.clone();
            if let Some(exe) = exe {
                let proc = self.process_manager.clone();
                thread::spawn(move || {
                    if let Ok(process) = proc.spawn(
                        app_name.clone(),
                        &exe,
                        &args.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                        &work_dir,
                    ) {
                        log_error!("PID", process.pid());
                        Self::update_pid(process.pid(), &app_name, app_config);
                        // send process
                        if let Ok(payload) = IpcHelper::compile(
                            EventPayload::Command.to_string(),
                            CmdrLog::pid(app_name, process.pid()),
                        ) {
                            _ = sender.send((
                                UserEvent::BroadcastToFrontEnd("main".to_string(), payload),
                                None,
                                None,
                            ));
                            proxy.wake_up();
                        };

                        while process.is_running() {
                            if let Ok(output) = process.try_recv() {
                                let response = CmdrLog::stdout(
                                    match output.level {
                                        cmd_manager::LogLevel::Stdout => LogType::Stdout,
                                        cmd_manager::LogLevel::Stderr => LogType::Stderr,
                                    },
                                    output.name.to_string(),
                                    output.message.to_string(),
                                );
                                if let Ok(payload) =
                                    IpcHelper::compile(EventPayload::Command.to_string(), response)
                                {
                                    _ = sender.send((
                                        UserEvent::BroadcastToFrontEnd("main".to_string(), payload),
                                        None,
                                        None,
                                    ));
                                    proxy.wake_up();
                                };
                            }
                            sleep(Duration::from_millis(10));
                        }
                        _ = process.kill();
                    }
                });
            }
        });

        Ok(())
    }
    pub fn kill_process(&self, app_name: String, sender: Sender<ChannelEvent>) {
        let app_config = {
            self.app_config
                .lock()
                .command_config()
                .commands
                .iter()
                .find(|c| c.name == app_name)
                .cloned()
        };
        if let Some(cmd) = app_config {
            let pid = cmd.pid;
            if let Some(status) = Command::new("taskkill")
                .args(["/PID", &pid.to_string(), "/F", "/T"])
                .status()
                .ok()
            {
                log_error!("KILL SUCCESS? ", status.success());
                Self::update_pid(0, &app_name, self.app_config.clone());
                if let Ok(payload) =
                    IpcHelper::compile(EventPayload::Command.to_string(), CmdrLog::pid(app_name, 0))
                {
                    _ = sender.send((
                        UserEvent::BroadcastToFrontEnd("main".to_string(), payload),
                        None,
                        None,
                    ));
                };
            }
        }
    }
    pub fn kill_all(&self) {
        _ = self.process_manager.kill_all();
    }
}
