#![allow(non_snake_case, clippy::empty_docs)]
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum EndCapData {
    S20(EndCapShape),
    S30(EndCapShape),
    S40(EndCapShape),
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum EndCapShape {
    Square,
    Arc,
    Rect,
}
