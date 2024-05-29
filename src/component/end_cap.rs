pub enum EndCapData {
    S20(EndCapShape),
    S30(EndCapShape),
    S40(EndCapShape),
}

pub enum EndCapShape {
    Square,
    Arc,
    Rect,
}