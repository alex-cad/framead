#![allow(non_snake_case, clippy::empty_docs)]
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum PanelData {
    Wood, // 木板
}
