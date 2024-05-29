pub enum FloorData {
    Wheel(WheelData),
    Foot(FootData),
}

pub enum WheelData {
    Fuma(FumaWheelData),
}

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

pub enum FootData {
    OrangeHeavy,
}
