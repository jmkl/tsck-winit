use crate::{event::TS_PATH, ts_struct};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

ts_struct! {path=TS_PATH,
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
