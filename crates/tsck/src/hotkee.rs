use std::{sync::Arc, time::Instant};

use flume::{Receiver, Sender, unbounded};
use kee::{Event, Expr, Kee, get_current_active_window, list_windows, parse_command};
use parking_lot::Mutex;
use winit::event_loop::EventLoopProxy;

use crate::event::{ChannelEvent, UserEvent};

enum SearchMode {
    Title,
    Name,
}

#[allow(unused)]
struct CycleApps {
    apps: Vec<String>,
    active_index: usize,
    active_workspace: usize,
}
impl CycleApps {
    pub fn new(apps: Vec<String>) -> Self {
        Self {
            apps,
            active_index: 0,
            active_workspace: 0,
        }
    }
    pub fn next(&mut self) -> usize {
        self.active_index = (self.active_index + 1) % self.apps.len();
        self.active_index
    }
    pub fn get_app(&self, index: usize) -> Option<&String> {
        self.apps.get(index)
    }
}
// zed,zen
pub fn spawn_hotkee(tx: Sender<ChannelEvent>, proxy: EventLoopProxy) {
    std::thread::spawn(move || {
        _spawn_hotkee(tx, &proxy);
    });
}

enum WindowOpsEvent {
    BringToFront(SearchMode, String),
    ToggleWorkspace,
}

struct WindowOps {
    active_workspace: Arc<Mutex<i8>>,
    receiver: Receiver<WindowOpsEvent>,
}

impl WindowOps {
    fn new(receiver: Receiver<WindowOpsEvent>) -> Self {
        Self {
            active_workspace: Arc::new(Mutex::new(0)),
            receiver,
        }
    }
    fn spawn(&self) {
        let receiver = self.receiver.clone();
        let active_workspace = self.active_workspace.clone();
        std::thread::spawn(move || {
            while let Ok(event) = receiver.recv() {
                match event {
                    WindowOpsEvent::BringToFront(search_mode, payload) => {
                        WindowOps::to_front(search_mode, &payload);
                    }
                    WindowOpsEvent::ToggleWorkspace => {
                        let workspace = {
                            let mut guard = active_workspace.lock();
                            *guard = if *guard == 1 { 0 } else { 1 };
                            *guard
                        };
                        if let Some(window) = get_current_active_window() {
                            println!("{:?}", window);
                        }
                        WindowOps::activate_workspace(workspace);
                    }
                }
            }
        });
    }
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

        if let Some(window) = kee::list_windows().iter().find(|k| match search_mode {
            SearchMode::Title => k.title().to_uppercase() == payload,
            SearchMode::Name => k.name().to_uppercase() == payload,
        }) {
            _ = window.bring_to_front();
        }
        println!("Execute in: {}ms", start.elapsed().as_millis());
    }
}

fn _spawn_hotkee(tx: Sender<ChannelEvent>, proxy: &EventLoopProxy) {
    let mut kee = Kee::new();
    let proxy = proxy.clone();
    let apps = Arc::new(Mutex::new(CycleApps::new(vec![
        "WHATSAPP.ROOT".to_string(),
        "PHOTOSHOP".to_string(),
        "T:GOOGLE".to_string(),
        "T:LLM".to_string(),
    ])));
    // let apps = Arc::new(Mutex::new(CycleApps::new(kee.get_apps())));
    let (winops_sender, winops_receiver) = unbounded::<WindowOpsEvent>();
    WindowOps::new(winops_receiver).spawn();
    let clone_apps = apps.clone();
    let app_sender = tx.clone();
    kee.on_message(move |event| match event {
        Event::Keys(_, f) => {
            if let Some(cmd) = parse_command(f) {
                proxy.wake_up();
                match cmd.namespace {
                    "app" => match cmd.action {
                        "CYCLE" => {
                            let mut clone_apps = clone_apps.lock();

                            let app = {
                                let index = clone_apps.next();
                                let app = clone_apps.get_app(index);
                                app
                            };
                            if let Some(app) = app {
                                if let Some((lhs, rhs)) = app.split_once(":") {
                                    if lhs == "T" {
                                        _ = winops_sender.send(WindowOpsEvent::BringToFront(
                                            SearchMode::Title,
                                            rhs.to_string(),
                                        ));
                                    }
                                } else {
                                    _ = winops_sender.send(WindowOpsEvent::BringToFront(
                                        SearchMode::Name,
                                        app.to_string(),
                                    ));
                                }
                            }
                        }
                        "PHOTOSHOP" => {
                            _ = winops_sender.send(WindowOpsEvent::BringToFront(
                                SearchMode::Name,
                                cmd.action.to_string(),
                            ));
                        }
                        "TSOCKEE" => match cmd.args.as_slice() {
                            [Expr::Ident(win_title)] => {
                                _ = winops_sender.send(WindowOpsEvent::BringToFront(
                                    SearchMode::Title,
                                    win_title.to_string(),
                                ));
                            }
                            _ => {}
                        },
                        "LAUNCHPLUGIN" => match cmd.args.as_slice() {
                            [Expr::Ident(win_title)] => {
                                _ = app_sender.send((
                                    UserEvent::LaunchPlugin(win_title.to_lowercase()),
                                    None,
                                ));
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    "workspace" => match cmd.action {
                        "TOGGLE" => {
                            _ = winops_sender.send(WindowOpsEvent::ToggleWorkspace);
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        _ => {}
    })
    .run();
}
