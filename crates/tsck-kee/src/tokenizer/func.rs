#![allow(unused)]
use crate::tokenizer::func_lexer::{Func, FuncExpr, FuncLexer};
use tsck_derive::{FuncParser, ScopeParser};

#[derive(Debug, FuncParser)]
enum WorkspaceFunc {
    MoveWindow,
    CycleWorkspace,
}

#[derive(Debug, FuncParser)]
enum AppFunc {
    Script(String),
    FuncCall(String),
    CyclePages(String),
    LaunchPlugin(String),
    ToggleWindowLevel,
    Page,
    CycleApps,
    ApptoFront,
    ReloadConfig,
    ToggleCompactMode,
    ToggleShadow,
}

#[derive(Debug, ScopeParser)]
enum FuncEntries {
    App(AppFunc),
    Workspace(WorkspaceFunc),
}

// "appfunc::Script("something")" to AppFunc::Script(String)

#[cfg(test)]
mod kee_func {
    use anyhow::Context;
    use std::{str::FromStr, time::Instant};

    use crate::tokenizer::{
        func::{AppFunc, FuncEntries, WorkspaceFunc},
        lexer::KeeParser,
    };
    #[test]
    fn test_lexer() -> anyhow::Result<()> {
        let input = include_str!("../../kee.kee");
        let lexer = KeeParser::new(input);
        let inputs = [
            "app::SCRIPT('splittext.js')",
            "app::FUNCCALL('createNewDocument')",
            "app::MOVEWINDOW",
        ];
        let start = Instant::now();
        for i in 0..1 {
            let kf = lexer.parse();
            let tk = kf.iter().as_ref().iter().map(|c| c.kee).collect::<Vec<_>>();
            //println!("{:?}", tk);
            for input in kf {
                if let Ok(entries) = FuncEntries::parse(input.func) {
                    match entries {
                        FuncEntries::App(app_func) => match app_func {
                            AppFunc::Script(_) => {
                                //eprintln!("AppFunc::Script");
                            }
                            AppFunc::FuncCall(_) => {
                                //eprintln!("AppFunc::FuncCall");
                            }
                            AppFunc::CyclePages(_) => {
                                //eprintln!("AppFunc::CyclePages");
                            }
                            AppFunc::LaunchPlugin(_) => {
                                //eprintln!("AppFunc::LaunchPlugin");
                            }
                            AppFunc::ToggleWindowLevel => {
                                //eprintln!("AppFunc::ToggleWindowLevel");
                            }
                            AppFunc::Page => {
                                //eprintln!("AppFunc::Page");
                            }
                            AppFunc::CycleApps => {
                                //eprintln!("AppFunc::CycleApps");
                            }
                            AppFunc::ApptoFront => {
                                //eprintln!("AppFunc::ApptoFront");
                            }
                            AppFunc::ReloadConfig => {
                                //eprintln!("AppFunc::ReloadConfig");
                            }
                            AppFunc::ToggleCompactMode => {
                                //eprintln!("AppFunc::ToggleCompactMode");
                            }
                            AppFunc::ToggleShadow => {
                                //eprintln!("AppFunc::ToggleShadow");
                            }
                        },
                        FuncEntries::Workspace(workspace_func) => match workspace_func {
                            WorkspaceFunc::MoveWindow => {
                                //eprintln!("WorkspaceFunc::MoveWindow");
                            }
                            WorkspaceFunc::CycleWorkspace => {
                                //eprintln!("WorkspaceFunc::CycleWorkspace");
                            }
                        },
                    }
                }
            }
        }
        println!("Execute in : {}micros", start.elapsed().as_micros());
        Ok(())
    }
}
