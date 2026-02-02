use crate::cmd::CommandConfig;
use crate::event::TS_PATH;
use crate::{DOTFILE_DIR, ts_struct};
use kee::TKeePair;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;
use tsck_utils::ConfigStore;

ts_struct! { path = TS_PATH,
    pub struct RawFilterDataType {
    pub temp: i32,
    pub tint: i32,
    pub texture: i32,
    pub clarity: i32,
    pub dehaze: i32,
    pub sharpen: i32,
    pub sharpen_radius: f32,
    pub sharpen_detail: i32,
    pub noise_reduction: i32,
    pub noise_reduction_detail: i32,
}
}

impl Default for RawFilterDataType {
    fn default() -> Self {
        Self {
            temp: 0,
            tint: 0,
            texture: 0,
            clarity: 0,
            dehaze: 0,
            sharpen: 0,
            sharpen_radius: 1.0,
            sharpen_detail: 25,
            noise_reduction: 0,
            noise_reduction_detail: 50,
        }
    }
}

ts_struct! { path = TS_PATH,
pub struct RawFilterTemplate {
    pub name: String,
    pub value: RawFilterDataType,
}
}

macro_rules! app_config {
	(
		$struct_name:ident => $struct_name_handler:ident,
		custom:  ($($custom_func:item)*),
		$($field:ident : $a:ty),+) => {
				#[derive(Serialize, Debug, Clone, Deserialize, Default,TS)]
				#[ts(export,export_to=TS_PATH)]
				#[serde(default)]
        pub struct $struct_name{
        	$(pub $field:$a),+
        }
        pub struct $struct_name_handler {
        	pub(crate) config_store:ConfigStore<AppConfig>
        }
        impl $struct_name_handler{
        	pub fn new()->Self{Self{config_store:Self::get()}}
        	pub fn update_config(&mut self,config:AppConfig){
         			_ = self.config_store.set(|c|*c=config);
         }
         	pub fn get()->ConfigStore<AppConfig>{ConfigStore::<AppConfig>::new(DOTFILE_DIR, "conf.json").expect("Failed to load conf.json")}
          $($custom_func)*
          $(
          pub fn $field(&self)->$a{
          	self.config_store.get(|c|c.$field.clone())
          }
          )+
        }
    };
}

app_config!(
    AppConfig =>AppConfigHandler,
    custom: (
        pub fn get_tkee_pair(&self)->Vec<TKeePair>{
            self.config_store.get(|c| {
                c.kees
                    .clone()
                    .into_iter()
                    .map(|(k, v)| TKeePair::new(k, v))
                    .collect()
            })
        }
    ),
    monitors							: Vec<(i32, i32)>,
    apps									: Vec<String>,
    pages									: Vec<String>,
    workspaces						: Vec<String>,
    kees									: HashMap<String, String>,
    version								: String,
    store_root						: String,
    http_server_port			: u16,
    whatsapp_bot_port			: u16,
    comfyui_url						: String,
    comfyui_root					: String,
    websocket_server_port	: u16,
    move_increment				: i32,
    resize_increment			: i32,
    rawfilter_template		: Vec<RawFilterTemplate>,
    command_config				: CommandConfig,
    color_list						: Vec<String>,
    whatsapp_url					: String

);

// #[derive(Serialize, Debug, Clone, Deserialize, Default)]
// #[serde(default)]
// pub struct AppConfig {
//     pub monitors: Vec<(i32, i32)>,
//     pub apps: Vec<String>,
//     pub pages: Vec<String>,
//     pub worskpace: Vec<String>,
//     pub kees: HashMap<String, String>,
//     pub version: String,
//     pub store_root: String,
//     pub http_server_port: u16,
//     pub websocket_server_port: u16,
//     pub move_increment: i32,
//     pub resize_increment: i32,
// }

// pub struct AppConfigHandler {
//     config_store: ConfigStore<AppConfig>,
// }
// impl AppConfigHandler {
//     pub fn new() -> Self {
//         Self {
//             config_store: Self::get(),
//         }
//     }
//     pub fn get() -> ConfigStore<AppConfig> {
//         ConfigStore::<AppConfig>::new(DOTFILE_DIR, "conf.json").expect("Failed to load conf.json")
//     }
//     pub fn get_tkee_pair(&self) -> Vec<TKeePair> {
//         self.config_store.get(|c| {
//             c.kees
//                 .clone()
//                 .into_iter()
//                 .map(|(k, v)| TKeePair::new(k, v))
//                 .collect()
//         })
//     }
//     pub fn monitors(&self) -> Vec<(i32, i32)> {
//         self.config_store.get(|c| c.monitors.clone())
//     }

//     pub fn apps(&self) -> Vec<String> {
//         self.config_store.get(|c| c.apps.clone())
//     }
//     pub fn pages(&self) -> Vec<String> {
//         self.config_store.get(|c| c.pages.clone())
//     }
//     pub fn workspace_apps(&self) -> Vec<String> {
//         self.config_store.get(|c| c.worskpace.clone())
//     }

//     pub fn kees(&self) -> HashMap<String, String> {
//         self.config_store.get(|c| c.kees.clone())
//     }

//     pub fn version(&self) -> String {
//         self.config_store.get(|c| c.version.clone())
//     }

//     pub fn store_root(&self) -> String {
//         self.config_store.get(|c| c.store_root.clone())
//     }

//     pub fn http_server_port(&self) -> u16 {
//         self.config_store.get(|c| c.http_server_port)
//     }

//     pub fn websocket_server_port(&self) -> u16 {
//         self.config_store.get(|c| c.websocket_server_port)
//     }
//     pub fn move_increment(&self) -> i32 {
//         self.config_store.get(|c| c.move_increment)
//     }
//     pub fn resize_increment(&self) -> i32 {
//         self.config_store.get(|c| c.resize_increment)
//     }
// }
