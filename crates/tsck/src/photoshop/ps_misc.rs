use crate::{event::TS_PATH, photoshop::ps_rawfilter::RawFilterDataType, ts_struct};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

ts_struct! { path = TS_PATH,
    (Default),
    pub struct TextLayerInfo {
        pub content: String,
        pub layer_id: i32,
        pub id: i32,
    }
}
ts_struct! {path = TS_PATH,(Default),
    pub struct RawFilterTextPipRange {
        pub rawfilter_data: RawFilterDataType,
        pub text_layers_info: Vec<TextLayerInfo>,
        pub pip_ranges: Vec<i32>,
        pub layer_kind: String,
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export,export_to = TS_PATH)]
#[ts(type = "(() => void)|undefined")]
#[allow(dead_code)]
pub struct UnListen;

ts_struct! {
    path = TS_PATH,(Default),
    pub struct Bounds {
        pub top: i32,
        pub bottom: i32,
        pub left: i32,
        pub right: i32,
    }
}
ts_struct! {path = TS_PATH,(Default),
    pub struct SelectionBound {
        pub selection_mode: bool,
        pub bounds: Bounds,
    }
}
