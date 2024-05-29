#[derive(Debug, Clone)]
pub enum EndCapData {
    S20(EndCapShape),
    S30(EndCapShape),
    S40(EndCapShape),
}

#[derive(Debug, Clone)]
pub enum EndCapShape {
    Square,
    Arc,
    Rect,
}
