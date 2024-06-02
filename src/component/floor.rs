#![allow(non_snake_case, clippy::empty_docs)]
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum FloorData {
    Wheel(WheelData),
    Foot(FootData),
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum WheelData {
    Fuma(FumaWheelData),
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum FumaWheelData {
    _40F,
    _60F,
    _80F,
    _100F,
    _120F,
    _150F,
    _40S,
    _60S,
    _80S,
    _100S,
    _120S,
    _150S,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum FootData {
    OrangeHeavy,
}
