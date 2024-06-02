use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EndCapData {
    S20(EndCapShape),
    S30(EndCapShape),
    S40(EndCapShape),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EndCapShape {
    Square,
    Arc,
    Rect,
}
