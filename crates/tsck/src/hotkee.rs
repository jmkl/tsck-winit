use std::time::Duration;
use std::{str::FromStr, sync::Arc, time::Instant};

use crate::event::ChannelEvent;
use crate::utils::animation::{AnimationEasing, map_value};
use crate::workspace_manager;
use crate::{app_config::AppConfigHandler, event::UserEvent};
use flume::Sender;
use parking_lot::Mutex;
use tsck_derive::{FuncParser, ScopeParser};
use tsck_kee::{AppInfo, AppPosition, Event, Kee, SafeHWND, TKeePair};
use tsck_kee::{Func, FuncExpr, FuncLexer};
use winit::event_loop::EventLoopProxy;

#[derive(Debug, FuncParser)]
enum WorkspaceFunc {
    Activate(i32),
    MoveActiveWindow(String),
    ResizeActiveWindow(String, String),
    CycleActiveWindowWidth,
    CycleActiveWindowPos,
    MoveActiveWindowToWorkspace,
    CycleWorkspace(String),
    WorkspaceTest,
}

#[derive(Debug, FuncParser)]
enum AppFunc {
    Script(String),
    FuncCall(String),
    CyclePages(String),
    LaunchPlugin(String),
    ToggleWindowLevel,
    Page(i32),
    CycleApps,
    AppToFront(String),
    ReloadConfig,
    ToggleCompactMode,
    ToggleShadow,
    AppTest,
}

#[derive(Debug, ScopeParser)]
enum FuncEntries {
    App(AppFunc),
    Workspace(WorkspaceFunc),
}
#[macro_export]
macro_rules! sender {
    ($channel_bus:ident,$page:ident  ) => {
        _ = $channel_bus.send((UserEvent::$page , None, None));
    };
    ($channel_bus:ident,$page:ident,  { $($fields:tt)* }  ) => {
        _ = $channel_bus.send((UserEvent::$page { $($fields)* }, None, None));
    };
    ($channel_bus:ident,$page:ident,$arg:expr) => {
        _ = $channel_bus.send((UserEvent::$page($arg), None, None));
    };
    ($channel_bus:ident,$page:ident,$arg:expr,$proxy:expr) => {
        _ = $channel_bus.send((UserEvent::$page($arg), None, None));
        $proxy.wake_up();
    };

}
macro_rules! match_ignore_case {
    (
        $value:expr,
        else : $else_result:expr,
        $( $pat:expr => $result:expr ),+ $(,)?
    ) => {
        match $value {
            $( l if $pat.eq_ignore_ascii_case($value) => $result, )+
            _=>$else_result
        }
    };
}

pub static ANIMATE_STATE: Mutex<bool> = parking_lot::const_mutex(false);
mod window_animation {
    use crate::hotkee::ANIMATE_STATE;
    pub fn is_running() -> bool {
        let mut anim_state = ANIMATE_STATE.lock();
        if !*anim_state {
            *anim_state = true;
            false
        } else {
            true
        }
    }
    pub fn animate_done() {
        *ANIMATE_STATE.lock() = false;
    }
}
#[derive(Debug)]
struct WorkspaceWindows {
    id: String,
    hwnd: SafeHWND,
    workspace: usize,
    real_position: AppPosition,
}

enum SearchMode {
    Title,
    Name,
}

#[allow(unused)]
struct WinOpsHandler {
    config_handler: AppConfigHandler,
    apps: Vec<String>,
    workspaces_apps: Vec<String>,
    active_index: usize,
    active_workspace: usize,
    ws_windows: Vec<WorkspaceWindows>,
}
macro_rules! cycle_value {
    ($which:expr, $max:expr) => {
        $which = ($which + 1) % $max;
    };
}
static WORKSPACE_LENGTH: usize = 3;
impl WinOpsHandler {
    pub fn new() -> Self {
        let config_handler = AppConfigHandler::new();
        let apps = config_handler.apps();
        let workspaces_apps = config_handler.workspaces();
        Self {
            config_handler,
            apps,
            workspaces_apps,
            active_index: 0,
            active_workspace: 0,
            ws_windows: Vec::new(),
        }
    }
    pub fn move_active_window_to_workspace(
        &mut self,
        sender: Sender<ChannelEvent>,
        proxy: Arc<EventLoopProxy>,
    ) {
        workspace_manager(|ws| {
            if let Some(active_window) = ws.get_active_app() {
                if ws.count_workspace() < 3 {
                    ws.add_new_workspace();
                }
                ws.move_app_to_workspace(&active_window);
                sender!(
                    sender,
                    WorkspaceSendPayload,
                    ws.get_workspace_payload(),
                    proxy
                );
            }
        });
        // if self
        //     .workspaces_apps
        //     .contains(&active_window.exe.to_uppercase())
        // {
        //     if let Some(window) = self
        //         .ws_windows
        //         .iter_mut()
        //         .find(|ws| ws.id == active_window.exe)
        //     {
        //         if window.workspace == self.active_workspace {
        //             window.real_position = active_window.position.clone();
        //         }
        //         log_debug!(dp!(window.real_position));
        //         window.workspace = (window.workspace + 1) % WORKSPACE_LENGTH;
        //         _ = active_window.move_to(
        //             window.real_position.x,
        //             window.real_position.y - (window.workspace as i32 * 1440),
        //         );
        //     }
        // }
    }
    pub fn cycle_workspace(&mut self, sender: Sender<ChannelEvent>, proxy: Arc<EventLoopProxy>) {
        workspace_manager(|ws| {
            ws.focus_next_app();
            // let active_workspace = ws.switch_to_next_workspace();
            sender!(
                sender,
                WorkspaceSendPayload,
                ws.get_workspace_payload(),
                proxy
            );
        });
        // cycle_value!(self.active_workspace, WORKSPACE_LENGTH);
        // self.ws_windows.iter().for_each(|w| {
        //     if self.active_workspace == w.workspace {
        //         _ = w.window.move_to(w.real_position.x, w.real_position.y);
        //     } else {
        //         _ = w
        //             .window
        //             .move_to(w.real_position.x, w.real_position.y - 1440);
        //     }
        // });
    }
    pub fn update_workspaces_app(&mut self, window_info: &AppInfo) {
        // let windows = list_windows();
        // if let Some(w) = windows.iter().find(|w| w.hwnd == window_info.hwnd) {
        //     self.active_window = Some(w.clone());
        // }
        // windows.iter().for_each(|w| {
        //     if self.workspaces_apps.contains(&w.name().to_uppercase()) {
        //         match self.ws_windows.iter_mut().find(|ws| ws.id == w.name()) {
        //             Some(window) => {
        //                 if window.workspace == self.active_workspace {
        //                     window.real_position = w.position().clone();
        //                 }
        //             }
        //             None => {
        //                 self.ws_windows.push(WorkspaceWindows {
        //                     id: w.name(),
        //                     hwnd: w.hwnd,
        //                     window: w.clone(),
        //                     workspace: self.active_workspace,
        //                     real_position: w.position().clone(),
        //                 });
        //             }
        //         }
        //     }
        // });
    }
    pub fn update_apps(&mut self) {
        self.config_handler = AppConfigHandler::new();
        self.apps = self.config_handler.apps();
    }
    pub fn next_app(&mut self, sender: Sender<ChannelEvent>) {
        self.active_index = (self.active_index + 1) % self.apps.len();
        if let Some(app) = self.get_app(self.active_index) {
            if let Some((lhs, rhs)) = app.split_once(":") {
                if lhs == "T" {
                    if rhs.eq_ignore_ascii_case("TSCK-BROWSER") {
                        sender!(sender, LaunchPlugin, rhs.to_lowercase());
                    } else {
                        WindowOps::to_front(SearchMode::Title, rhs);
                    }
                }
            } else {
                WindowOps::to_front(SearchMode::Name, app);
            }
        }
    }
    // pub fn next_workspace(&mut self) -> usize {
    //     self.active_workspace = (self.active_workspace + 1) % 3;
    //     self.active_workspace
    // }

    pub fn get_app(&self, index: usize) -> Option<&String> {
        self.apps.get(index)
    }
    fn cycle_move_active_app(&mut self, app: &AppInfo) {
        if self
            .workspaces_apps
            .iter()
            .find(|w| app.exe.to_uppercase() == w.to_uppercase())
            .is_some()
        {
            cycle_value!(self.active_workspace, WORKSPACE_LENGTH);
            let active_workspace = self.active_workspace as i32;
            //zed -8 -8 (2576, 1408)
            let monitor = 2576 / 3;
            _ = tsck_kee::api::app_move_to(app, -8 + (monitor * active_workspace), -8);
        }
    }
    fn arrange_window_on_workspace(&self) {
        //TODO must retrieve workspace from the api and update accordingly
        let workspace = 0;
        let count = self
            .ws_windows
            .iter()
            .filter(|w| w.workspace == self.active_workspace)
            .count();
        if count > 1 {
            //zed -8 -8 (2576, 1408)
            let width = 2576 / count;
            let height = 1408;
            for w in tsck_kee::api::app_get_all() {
                _ = tsck_kee::api::app_resize(&w, width as i32, height);
                _ = tsck_kee::api::app_move_to(&w, (width * workspace) as i32, 0);
            }
        }
    }
    fn cycle_resize_active_app(&mut self, window: &AppInfo) {
        let whitelist = true;
        if whitelist {
            cycle_value!(self.active_workspace, WORKSPACE_LENGTH);
            let active_workspace = self.active_workspace as i32;
            let monitor = (2576, 1408);

            //zed -8 -8 (2576, 1408)

            // log_debug!(window.name(), dp!(window.size()), dp!(window.position()));
            match active_workspace {
                0 => {
                    _ = tsck_kee::api::app_resize(window, monitor.0 / 3, monitor.1);
                    _ = tsck_kee::api::app_move_to(window, -8, -8);
                }
                1 => {
                    _ = tsck_kee::api::app_resize(window, monitor.0 / 2, monitor.1);
                    _ = tsck_kee::api::app_move_to(window, -8, -8);
                }
                2 => {
                    _ = tsck_kee::api::app_resize(window, monitor.0, monitor.1);
                    _ = tsck_kee::api::app_move_to(window, -8, -8);
                }
                _ => {}
            }
        }
    }
}

struct WindowOps;

impl WindowOps {
    fn to_front(search_mode: SearchMode, payload: &str) {
        if let Some(window) = tsck_kee::list_windows().iter().find(|k| match search_mode {
            SearchMode::Title => k.title().to_uppercase() == payload.to_uppercase(),
            SearchMode::Name => k.name().to_uppercase() == payload.to_uppercase(),
        }) {
            _ = window.bring_to_front();
        }
    }

    fn move_active_app(to: &str) {
        if window_animation::is_running() {
            return;
        }

        let to = to.to_string();
        std::thread::spawn(move || {
            let inc = AppConfigHandler::new().move_increment();
            if let Some(w) = tsck_kee::api::app_get_active() {
                let (wx, wy) = (w.position.x, w.position.y);

                let (a, b) = match_ignore_case!(&to,
                      else : (wx,wy),
                      "LEFT"  => (wx - inc, wy),
                      "RIGHT" => (wx + inc, wy),
                      "UP"    => (wx, wy - inc),
                      "DOWN"  => (wx, wy + inc),
                );
                // _ = w.move_to(a, b);
                WindowOps::animate_app(
                    &w,
                    Some((a, b)),
                    None,
                    150,
                    AnimationEasing::EaseInOutCubic,
                );
            }
        });
    }
    fn cycle_active_app_width(arc_channel_bus: Sender<ChannelEvent>, proxy: Arc<EventLoopProxy>) {
        workspace_manager(|wm| {
            if let Some(active_app) = wm.get_active_app() {
                let index = 0;
                sender!(arc_channel_bus, WorkspaceAppFocusChange, index as i32);
            }
        });
    }
    fn resize_active_app(prop: &str, increment: bool) {
        if window_animation::is_running() {
            return;
        }
        let prop = prop.to_string();
        std::thread::spawn(move || {
            let inc = {
                let increment_value = AppConfigHandler::new().move_increment();
                if increment {
                    increment_value
                } else {
                    -increment_value
                }
            };
            if let Some(w) = tsck_kee::api::app_get_active() {
                let (width, height) = (w.size.width, w.size.height);
                let (a, b) = match_ignore_case!(&prop,
                    else: (width, height),
                    "WIDTH" => (width + inc, height),
                    "HEIGHT" => (width, height + inc),
                );
                _ = tsck_kee::api::app_resize(&w, a, b);
                window_animation::animate_done();
                // WindowOps::animate_window(
                //     &w,
                //     None,
                //     Some((a, b)),
                //     150,
                //     AnimationEasing::EaseInOutCubic,
                // );
            }
        });
    }
    fn animate_app(
        ws: &AppInfo,
        to_pos: Option<(i32, i32)>,
        to_size: Option<(i32, i32)>,
        duration: u64,
        easing: AnimationEasing,
    ) {
        let pos = ws.position.clone();
        let size = ws.size.clone();
        let to_pos = to_pos.unwrap_or((pos.x, pos.y));
        let to_size = to_size.unwrap_or((size.width as i32, size.height as i32));
        let start_time = Instant::now();
        let duration = Duration::from_millis(duration);
        const TARGET_FPS: u64 = 120;
        const FRAME_TIME: Duration = Duration::from_millis(1000 / TARGET_FPS);
        while start_time.elapsed() < duration {
            let frame_start = Instant::now();

            let t = start_time.elapsed().as_secs_f64() / duration.as_secs_f64();
            let eased_t = easing.evaluate(t.min(1.0));

            // Only call the operations that are actually changing
            if to_pos != (pos.x, pos.y) {
                let new_pos = map_value((pos.x, pos.y), (to_pos.0, to_pos.1), eased_t);
                _ = tsck_kee::api::app_move_to(ws, new_pos.0 as i32, new_pos.1 as i32);
            }

            if to_size != (size.width as i32, size.height as i32) {
                let new_size = map_value(
                    (size.width as i32, size.height as i32),
                    (to_size.0, to_size.1),
                    eased_t,
                );
                _ = tsck_kee::api::app_resize(ws, new_size.0 as i32, new_size.1 as i32);
            }

            // Sleep only for remaining frame time
            let elapsed = frame_start.elapsed();
            if elapsed < FRAME_TIME {
                std::thread::sleep(FRAME_TIME - elapsed);
            }
        }

        _ = tsck_kee::api::app_move_to(ws, to_pos.0 as i32, to_pos.1 as i32);
        _ = tsck_kee::api::app_resize(ws, to_size.0 as i32, to_size.1 as i32);
        window_animation::animate_done();
    }
}

pub fn __spawn_hotkee(
    proxy: Arc<EventLoopProxy>,
    sender: Sender<ChannelEvent>,
) -> anyhow::Result<()> {
    let config = AppConfigHandler::new();
    let kees: Vec<TKeePair> = config.get_tkee_pair();
    let winops_handler = Arc::new(Mutex::new(WinOpsHandler::new()));
    let kee = Arc::new(Mutex::new(Kee::new(true)));
    let arc_channel_bus = sender.clone();
    let arc_kee = kee.clone();
    let arc_winops_handler = winops_handler.clone();

    //instantiate workspace ui
    workspace_manager(|wm| {
        sender!(
            arc_channel_bus,
            WorkspaceSendPayload,
            wm.get_workspace_payload()
        );
    });

    kee.lock()
        .on_message(move |event| match event {
            Event::Keys(_, f) => {
                proxy.wake_up();
                if let Ok(entries) = FuncEntries::from_str(f) {
                    match entries {
                        FuncEntries::App(func) => match func {
                            AppFunc::ReloadConfig => {
                                {
                                    arc_winops_handler.lock().update_apps();
                                }
                                _ = arc_kee
                                    .clone()
                                    .lock()
                                    .update_hotkeys(AppConfigHandler::new().get_tkee_pair());
                            }
                            AppFunc::LaunchPlugin(win_title) => {
                                sender!(arc_channel_bus, LaunchPlugin, win_title.to_lowercase());
                            }
                            AppFunc::AppToFront(what) => {
                                WindowOps::to_front(SearchMode::Name, &what);
                            }
                            AppFunc::CycleApps => {
                                arc_winops_handler.lock().next_app(arc_channel_bus.clone());
                            }
                            AppFunc::CyclePages(direction) => {
                                sender!(
                                    arc_channel_bus,
                                    CyclePages,
                                    match direction.as_str() {
                                        "PREV" => -1,
                                        "NEXT" => 1,
                                        _ => 0,
                                    }
                                );
                            }
                            AppFunc::Page(page) => {
                                sender!(arc_channel_bus, FocusPage, page);
                            }
                            AppFunc::Script(script) => {
                                sender!(arc_channel_bus, ExecuteScript, script);
                            }
                            AppFunc::FuncCall(func) => {
                                sender!(arc_channel_bus, FunctionCall, { func, args: vec![] });
                            }
                            AppFunc::ToggleShadow => {
                                sender!(arc_channel_bus, ToggleShadow);
                            }
                            AppFunc::ToggleWindowLevel => {
                                sender!(arc_channel_bus, ToggleWindowLevel);
                            }
                            AppFunc::ToggleCompactMode => {
                                sender!(arc_channel_bus, ToggleCompactMode);
                            }
                            AppFunc::AppTest => todo!(),
                        },
                        FuncEntries::Workspace(func) => match func {
                            WorkspaceFunc::Activate(page) => {
                                workspace_manager(|wm| {
                                    sender!(
                                        arc_channel_bus,
                                        WorkspaceSendPayload,
                                        wm.get_workspace_payload()
                                    );
                                });
                            }
                            WorkspaceFunc::MoveActiveWindow(to) => {
                                WindowOps::move_active_app(&to);
                            }
                            WorkspaceFunc::ResizeActiveWindow(inc, prop) => {
                                let increment = inc.to_uppercase().as_str() == "INC";
                                WindowOps::resize_active_app(&prop, increment)
                            }
                            WorkspaceFunc::CycleActiveWindowWidth => {
                                {
                                    WindowOps::cycle_active_app_width(
                                        arc_channel_bus.clone(),
                                        proxy.clone(),
                                    );
                                }
                                {
                                    // let index = { arc_winops_handler.lock().next_workspace() };
                                    // sender!(arc_channel_bus, ActivateWorkSpace, index as i32);
                                    // proxy.wake_up();
                                }
                            }
                            WorkspaceFunc::CycleActiveWindowPos => {
                                {
                                    WindowOps::cycle_active_app_width(
                                        arc_channel_bus.clone(),
                                        proxy.clone(),
                                    );
                                    // let index = { arc_winops_handler.lock().next_workspace() };
                                    // sender!(arc_channel_bus, ActivateWorkSpace, index as i32);
                                    // proxy.wake_up();
                                }
                            }
                            WorkspaceFunc::CycleWorkspace(direction) => {
                                workspace_manager(|ws| {
                                    match direction.as_str() {
                                        "next" => {
                                            ws.focus_next_app();
                                        }
                                        "prev" => {
                                            ws.focus_prev_app();
                                        }
                                        _ => {}
                                    }

                                    sender!(
                                        sender,
                                        WorkspaceSendPayload,
                                        ws.get_workspace_payload(),
                                        proxy
                                    );
                                });
                            }
                            WorkspaceFunc::MoveActiveWindowToWorkspace => {
                                workspace_manager(|wm| if let Some(app) = wm.get_active_app() {});

                                arc_winops_handler.lock().move_active_window_to_workspace(
                                    arc_channel_bus.clone(),
                                    proxy.clone(),
                                );
                            }
                            WorkspaceFunc::WorkspaceTest => {
                                tsck_kee::api::maximize_window();
                            }
                        },
                    }
                }
            }

            _ => {}
        })
        .run(kees);
    Ok(())
}

#[cfg(test)]
mod bool_test {
    use std::sync::atomic::AtomicBool;

    #[test]
    fn atomic_bool() {
        let state = AtomicBool::new(false);
        let status = state.load(std::sync::atomic::Ordering::Relaxed);
        println!("INIT {}", status);
        state.swap(true, std::sync::atomic::Ordering::Relaxed);
        let status = state.load(std::sync::atomic::Ordering::Relaxed);
        println!("SWAP TO TRUE {}", status);
        state.store(false, std::sync::atomic::Ordering::Relaxed);
        let status = state.load(std::sync::atomic::Ordering::Relaxed);
        println!("STORE TO FALSE {}", status);
        if status {
            return;
        }

        std::thread::sleep(std::time::Duration::from_millis(300));
    }
}
