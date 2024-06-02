use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtrudeData {
    pub(crate) standard: ExtrudeStandard,
    pub(crate) shape: ExtrudeShape,
    pub(crate) post_process: ExtrudePostProcess,
}

// 铝型材标准
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExtrudeStandard {
    pub(crate) series: ExtrudeSeries,   // 铝型材系列
    pub(crate) metarial: Metarial,      // 铝型材材质
    pub(crate) surface: ExtrudeSurface, // 铝型材表面处理
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum ExtrudeSurface {
    // AA5,  // 阳极氧化 5um层厚
    AA10, // 阳极氧化 10um层厚
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum Metarial {
    _6063T5, // 牌号 6063 状态 T5
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum ExtrudeSeries {
    S20(),                    // 20系列 6mm槽宽
    S30(),                    // 30系列 8mm槽宽
    S40(S40ExtrudeSlotDepth), // 40系列 8mm槽宽
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum S40ExtrudeSlotDepth {
    SlotDepth14_7mm, // 槽深14.7mm
    SlotDepth12_3mm, // 槽深12.3mm
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExtrudeShape {
    pub(crate) name: String,
    pub(crate) shape: ExtrudeShapeEnum,
    pub(crate) holes_count: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum ExtrudeShapeEnum {
    Square(ExtrudeSquareShape),

    // 长边/短边
    Rect(u8, ExtrudeRectShape),
    // L(u8),
    // Arc(u8),
    // Angle(ExtrudeAngle),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum ExtrudeSquareShape {
    FourSlot,
    ThreeSlot,           // 三个槽，平面朝下
    TwoSlotOppositeSide, // 对角两个槽, 平面处于上下两个位置
    OneSlot,             // 一个槽，槽向上
    Arc,                 // 圆弧面朝左上角
    Bevel,               // 斜面朝左上角
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum ExtrudeRectShape {
    // 长方形竖向放置，平面朝右
    FourSlot,
    ThreeSlot,
    TwoSlot,
    TwoSlotOppositeSide,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ExtrudeCutDirection {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExtrudePostProcess {
    pub(crate) drill: Drill,                      // 钻孔
    pub(crate) bevel_cut: bool,                   // 斜切
    pub(crate) wrench_hole: bool,                 // 扳手孔
    pub(crate) wrench_hole_size: u8,              // 扳手孔尺寸
    pub(crate) counterbore: bool,                 // 沉头孔 (一个方向最多5个)
    pub(crate) counterbore_size: CounterboreSize, // 沉头孔尺寸
    pub(crate) length: ExtrudeLength,             // 长度
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExtrudeLength {
    pub(crate) min: u32,  // 0.01mm
    pub(crate) max: u32,  // 0.01mm
    pub(crate) step: u32, // 0.01mm
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum Drill {
    M6_15mm,
    M8_20mm,
    M8_25mm,
    M12_30mm,
    M14_30mm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum CounterboreSize {
    // d = 11mm d1 = 6.6mm
    Z6,

    // d = 14mm d1 = 9mm
    Z8,

    // d = 19mm d1 = 13mm
    Z12,
}
