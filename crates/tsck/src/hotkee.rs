use std::{str::FromStr, sync::Arc, time::Instant};

use crate::{
    ChannelBus,
    app_config::AppConfigHandler,
    event::{ReadableHotkee, UserEvent},
    log_error,
};
use parking_lot::Mutex;
use tsck_kee::{Event, Kee, SafeHWND, TKeePair, WindowInfo, list_windows};
use tsck_utils::{Expr, generate_func_enums, parse_func};

generate_func_enums!(
    KeeEntry => (
        App => (
            LaunchPlugin,
            AppToFront,
            CycleApps,
            ReloadConfig,
            CyclePages,
            Page,
            Script,
            FuncCall,
            ToggleShadow,
            ToggleWindowLevel,
            ToggleCompactMode


        )
        Workspace => (
            CycleWorkSpace,
            Activate,
            MoveWindow,
            MoveWindowToWorkSpace
        )
    )
);

/// @variant
/// Expr::String
/// Expr::Ident
/// Expr::Number
/// Expr::Call
/// Expr::Tuple
macro_rules! slice_args {
    ($cmd:expr,$variant:path, |$param:ident| $body:block) => {
        match $cmd.args.as_slice() {
            [$variant($param)] => $body,
            _ => {}
        }
    };
}

enum SearchMode {
    Title,
    Name,
}

#[allow(unused)]
struct WindowOpsHandler {
    config_handler: AppConfigHandler,
    apps: Vec<String>,
    active_index: usize,
    active_workspace: usize,
    active_window: Option<WindowInfo>,
}
impl WindowOpsHandler {
    pub fn new() -> Self {
        let config_handler = AppConfigHandler::new();
        let apps = config_handler.apps();
        Self {
            config_handler,
            apps,
            active_index: 0,
            active_workspace: 0,
            active_window: None,
        }
    }
    pub fn update_apps(&mut self) {
        self.config_handler = AppConfigHandler::new();
        self.apps = self.config_handler.apps();
    }
    pub fn next_app(&mut self) {
        self.active_index = (self.active_index + 1) % self.apps.len();
        if let Some(app) = self.get_app(self.active_index) {
            if let Some((lhs, rhs)) = app.split_once(":") {
                if lhs == "T" {
                    WindowOps::to_front(SearchMode::Title, rhs);
                }
            } else {
                WindowOps::to_front(SearchMode::Name, app);
            }
        }
    }
    pub fn next_workspace(&mut self) -> usize {
        self.active_workspace = (self.active_workspace + 1) % 3;
        self.active_workspace
    }
    pub fn set_active_window(&mut self, window_info: WindowInfo) {
        self.active_window = Some(window_info);
    }
    pub fn get_active_window(&self) -> &Option<WindowInfo> {
        &self.active_window
    }
    pub fn get_app(&self, index: usize) -> Option<&String> {
        self.apps.get(index)
    }
}
// zed,zen
pub fn init_hotkee(bus: Arc<ChannelBus>) {
    std::thread::spawn(move || {
        if let Err(err) = _spawn_hotkee(bus) {
            log_error!("ERROR HOTKEY", err);
        }
    });
}

struct WindowOps;

impl WindowOps {
    fn activate_workspace(which: i8) {
        for w in list_windows().iter() {
            if which == 0 {
                match w.name().as_str() {
                    "zen" => {
                        _ = w.move_to(w.position().x, w.position().y - 1440);
                    }
                    _ => {}
                }
            } else {
                match w.name().as_str() {
                    "zen" => {
                        _ = w.move_to(w.position().x, w.position().y + 1440);
                        _ = w.bring_to_front();
                    }
                    _ => {}
                }
            }
        }
    }
    fn to_front(search_mode: SearchMode, payload: &str) {
        let start = Instant::now();

        if let Some(window) = tsck_kee::list_windows().iter().find(|k| match search_mode {
            SearchMode::Title => k.title().to_uppercase() == payload.to_uppercase(),
            SearchMode::Name => k.name().to_uppercase() == payload.to_uppercase(),
        }) {
            _ = window.bring_to_front();
        }
        println!("Execute in: {}ms", start.elapsed().as_millis());
    }
    fn move_window(to: &str, hwnd: &SafeHWND) {
        let inc = AppConfigHandler::new().move_increment();
        if let Some(w) = list_windows().iter().find(|w| &w.hwnd == hwnd) {
            let (wx, wy) = (w.position().x, w.position().y);
            match to {
                "LEFT" => {
                    _ = w.move_to(wx - inc, wy);
                }
                "RIGHT" => {
                    _ = w.move_to(wx + inc, wy);
                }
                "UP" => {
                    _ = w.move_to(wx, wy - inc);
                }
                "DOWN" => {
                    _ = w.move_to(wx, wy + inc);
                }
                _ => {}
            }
        }
    }
}
pub fn kee_to_readable_hotkee(input: &str, func: &str) -> ReadableHotkee {
    let parts: Vec<&str> = input.split('-').collect();
    let mut kee = ReadableHotkee::default();
    let key_part = if parts.len() == 1 {
        parts[0]
    } else {
        for part in &parts[..parts.len() - 1] {
            match part.as_bytes() {
                b"C" => kee.ctrl = true,
                b"S" => kee.shift = true,
                b"A" => kee.alt = true,
                b"M" | b"W" => kee.meta = true,
                _ => {}
            }
        }
        parts[parts.len() - 1]
    };
    kee.key = key_part.to_string();
    kee.func = func.to_string();
    kee
}

fn _spawn_hotkee(bus: Arc<ChannelBus>) -> anyhow::Result<()> {
    let config = AppConfigHandler::new();
    let kees: Vec<TKeePair> = config.get_tkee_pair();
    let apps = Arc::new(Mutex::new(WindowOpsHandler::new()));
    let apps = apps.clone();
    let channel_bus = bus.clone();
    let kee = Kee::new();
    let kee = Arc::new(Mutex::new(kee));
    let key_for_message = kee.clone();

    kee.lock()
        .on_message(move |event| match event {
            Event::Keys(_, f) => {
                if let Some(cmd) = parse_func(f) {
                    channel_bus.wake_up();
                    if let Ok(entry) = KeeEntry::from_str(cmd.entry) {
                        match entry {
                            KeeEntry::App => {
                                if let Ok(func) = AppFunc::from_str(cmd.func) {
                                    match func {
                                        AppFunc::ReloadConfig => {
                                            {
                                                apps.lock().update_apps();
                                            }
                                            _ = key_for_message.clone().lock().update_hotkeys(
                                                AppConfigHandler::new().get_tkee_pair(),
                                            );
                                        }

                                        AppFunc::LaunchPlugin => {
                                            slice_args!(cmd, Expr::Ident, |win_title| {
                                                _ = channel_bus.send((
                                                    UserEvent::LaunchPlugin(
                                                        win_title.to_lowercase(),
                                                    ),
                                                    None,
                                                    None,
                                                ));
                                            });
                                        }

                                        AppFunc::AppToFront => {
                                            slice_args!(cmd, Expr::Ident, |what| {
                                                WindowOps::to_front(SearchMode::Name, what);
                                            });
                                        }
                                        AppFunc::CycleApps => {
                                            apps.lock().next_app();
                                        }
                                        AppFunc::CyclePages => {
                                            slice_args!(cmd, Expr::Ident, |direction| {
                                                let direction = match *direction {
                                                    "PREV" => -1,
                                                    "NEXT" => 1,
                                                    _ => 0,
                                                };
                                                _ = channel_bus.send((
                                                    UserEvent::CyclePages(direction),
                                                    None,
                                                    None,
                                                ));
                                            })
                                        }

                                        AppFunc::Page => slice_args!(cmd, Expr::Number, |page| {
                                            _ = channel_bus.send((
                                                UserEvent::FocusPage(*page as i32),
                                                None,
                                                None,
                                            ));
                                        }),
                                        AppFunc::Script => {
                                            slice_args!(cmd, Expr::String, |script| {
                                                _ = channel_bus.send((
                                                    UserEvent::ExecuteScript(script.to_string()),
                                                    None,
                                                    None,
                                                ));
                                            })
                                        }
                                        AppFunc::FuncCall => {
                                            slice_args!(cmd, Expr::Ident, |func| {
                                                _ = channel_bus.send((
                                                    UserEvent::FunctionCall {
                                                        func: func.to_string(),
                                                        args: vec![],
                                                    },
                                                    None,
                                                    None,
                                                ));
                                            })
                                        }
                                        AppFunc::ToggleShadow => {
                                            _ = channel_bus.send((
                                                UserEvent::ToggleShadow,
                                                None,
                                                None,
                                            ));
                                        }
                                        AppFunc::ToggleWindowLevel => {
                                            _ = channel_bus.send((
                                                UserEvent::ToggleWindowLevel,
                                                None,
                                                None,
                                            ));
                                        }
                                        AppFunc::ToggleCompactMode => {
                                            _ = channel_bus.send((
                                                UserEvent::ToggleCompactMode,
                                                None,
                                                None,
                                            ));
                                        }
                                    }
                                }
                            }
                            KeeEntry::Workspace => {
                                if let Ok(func) = WorkspaceFunc::from_str(cmd.func) {
                                    match func {
                                        WorkspaceFunc::CycleWorkSpace => {
                                            {
                                                let mut clone_apps = apps.lock();
                                                let index = clone_apps.next_workspace();
                                                _ = channel_bus.send((
                                                    UserEvent::ActivateWorkSpace(index as i64),
                                                    None,
                                                    None,
                                                ));
                                            };
                                        }
                                        WorkspaceFunc::Activate => match cmd.args.as_slice() {
                                            [Expr::Number(page)] => {
                                                _ = channel_bus.send((
                                                    UserEvent::ActivateWorkSpace(*page),
                                                    None,
                                                    None,
                                                ));
                                            }
                                            _ => {}
                                        },
                                        WorkspaceFunc::MoveWindow => match cmd.args.as_slice() {
                                            [Expr::Ident(to)] => {
                                                let clone_apps = apps.clone();
                                                {
                                                    if let Some(w) =
                                                        clone_apps.lock().get_active_window()
                                                    {
                                                        WindowOps::move_window(*to, &w.hwnd);
                                                    }
                                                }
                                            }
                                            _ => {}
                                        },
                                        WorkspaceFunc::MoveWindowToWorkSpace => {
                                            match cmd.args.as_slice() {
                                                [Expr::Ident(to)] => {}
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Event::WindowChange(safe_window_info) => {
                if let Some(w) = list_windows()
                    .iter()
                    .find(|w| w.hwnd == safe_window_info.hwnd)
                {
                    let clone_apps = apps.clone();
                    {
                        clone_apps.lock().set_active_window(w.clone());
                    }
                }
            }
            _ => {}
        })
        .run(kees);
    Ok(())
}
