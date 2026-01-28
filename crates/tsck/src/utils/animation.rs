#![allow(unused)]
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::event::TS_PATH;

#[derive(Deserialize, TS, Debug, Serialize, Clone, PartialEq)]
#[ts(export_to=TS_PATH)]
pub enum AnimationEasing {
    EaseInSine,
    EaseOutSine,
    EaseInOutSine,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,
    EaseInQuint,
    EaseOutQuint,
    EaseInOutQuint,
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,
    EaseInCirc,
    EaseOutCirc,
    EaseInOutCirc,
    EaseOutBack,
    EaseInOutBack,
    EaseOutElastic,
    EaseOutBounce,
    EaseInBounce,
}
impl AnimationEasing {
    pub fn evaluate(&self, t: f64) -> f64 {
        match self {
            AnimationEasing::EaseInSine => 1.0 - (t * std::f64::consts::FRAC_PI_2).cos(),
            AnimationEasing::EaseOutSine => (t * std::f64::consts::FRAC_PI_2).sin(),
            AnimationEasing::EaseInOutSine => -((t * std::f64::consts::PI).cos() - 1.0) / 2.0,
            AnimationEasing::EaseInQuad => t * t,
            AnimationEasing::EaseOutQuad => 1.0 - (1.0 - t) * (1.0 - t),
            AnimationEasing::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            AnimationEasing::EaseInCubic => t * t * t,
            AnimationEasing::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            AnimationEasing::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            AnimationEasing::EaseInQuart => t * t * t * t,
            AnimationEasing::EaseOutQuart => 1.0 - (1.0 - t).powi(4),
            AnimationEasing::EaseInOutQuart => {
                if t < 0.5 {
                    8.0 * t * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
                }
            }
            AnimationEasing::EaseInQuint => t * t * t * t * t,
            AnimationEasing::EaseOutQuint => 1.0 - (1.0 - t).powi(5),
            AnimationEasing::EaseInOutQuint => {
                if t < 0.5 {
                    16.0 * t * t * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(5) / 2.0
                }
            }
            AnimationEasing::EaseInExpo => {
                if t == 0.0 {
                    0.0
                } else {
                    2.0f64.powf(10.0 * t - 10.0)
                }
            }
            AnimationEasing::EaseOutExpo => {
                if t == 1.0 {
                    1.0
                } else {
                    1.0 - 2.0f64.powf(-10.0 * t)
                }
            }
            AnimationEasing::EaseInOutExpo => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else if t < 0.5 {
                    2.0f64.powf(20.0 * t - 10.0) / 2.0
                } else {
                    (2.0 - 2.0f64.powf(-20.0 * t + 10.0)) / 2.0
                }
            }
            AnimationEasing::EaseInCirc => 1.0 - (1.0 - t * t).sqrt(),
            AnimationEasing::EaseOutCirc => (1.0 - (t - 1.0).powi(2)).sqrt(),
            AnimationEasing::EaseInOutCirc => {
                if t < 0.5 {
                    (1.0 - (1.0 - (2.0 * t).powi(2)).sqrt()) / 2.0
                } else {
                    ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0
                }
            }
            AnimationEasing::EaseOutBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
            }
            AnimationEasing::EaseInOutBack => {
                let c1 = 1.70158;
                let c2 = c1 * 1.525;
                if t < 0.5 {
                    ((2.0 * t).powi(2) * ((c2 + 1.0) * 2.0 * t - c2)) / 2.0
                } else {
                    ((2.0 * t - 2.0).powi(2) * ((c2 + 1.0) * (t * 2.0 - 2.0) + c2) + 2.0) / 2.0
                }
            }
            AnimationEasing::EaseOutElastic => {
                const C4: f64 = (2.0 * std::f64::consts::PI) / 3.0;

                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    (2.0f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * C4).sin()) + 1.0
                }
            }
            AnimationEasing::EaseOutBounce => {
                let n1 = 7.5625;
                let d1 = 2.75;

                if t < 1.0 / d1 {
                    n1 * t * t
                } else if t < 2.0 / d1 {
                    let t = t - 1.5 / d1;
                    n1 * t * t + 0.75
                } else if t < 2.5 / d1 {
                    let t = t - 2.25 / d1;
                    n1 * t * t + 0.9375
                } else {
                    let t = t - 2.625 / d1;
                    n1 * t * t + 0.984375
                }
            }
            AnimationEasing::EaseInBounce => 1.0 - AnimationEasing::EaseOutBounce.evaluate(1.0 - t),
        }
    }
}
pub fn map_value(start: (i32, i32), end: (i32, i32), eased_t: f64) -> (f64, f64) {
    let new_x = start.0 as f64 + (end.0 - start.0) as f64 * eased_t;
    let new_y = start.1 as f64 + (end.1 - start.1) as f64 * eased_t;
    (new_x, new_y)
}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to=TS_PATH)]
pub struct AnimationPayload {
    pub label: String,
    #[ts(type = "number")]
    pub duration: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub to_size: Option<(i32, i32)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub to_pos: Option<(i32, i32)>,
    pub easing: AnimationEasing,
}
impl AnimationPayload {
    pub fn builder(window_label: impl Into<String>) -> AnimationPayload {
        AnimationPayload {
            label: window_label.into(),
            duration: 250,
            to_size: None,
            to_pos: None,
            easing: AnimationEasing::EaseOutExpo,
        }
    }
    pub fn to_size(mut self, size: (i32, i32)) -> AnimationPayload {
        self.to_size = Some((size.0, size.1));
        self
    }
    pub fn to_position(mut self, position: (i32, i32)) -> AnimationPayload {
        self.to_pos = Some((position.0, position.1));
        self
    }
    pub fn duration(mut self, duration: u64) -> AnimationPayload {
        self.duration = duration;
        self
    }
    pub fn easing(mut self, easing: AnimationEasing) -> AnimationPayload {
        self.easing = easing;
        self
    }
}

impl Default for AnimationPayload {
    fn default() -> Self {
        Self {
            label: "main".to_string(),
            duration: 250,
            to_size: None,
            to_pos: None,
            easing: AnimationEasing::EaseInSine,
        }
    }
}
