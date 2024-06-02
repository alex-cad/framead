#![allow(non_snake_case, clippy::empty_docs)]
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum ExtrudeConnectorData {
    Bracket(BracketData),                 // 角码
    SlotBracket(SlotBracketData),         // 槽连接件
    ConnectorPlate(ConnectorPlateData),   // 连接板
    Nut(NutData),                         // 螺母
    Bolt(BoltData),                       // 螺栓
    ElasticFastener(ElasticFastenerData), // 弹性扣件
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub struct BracketData {
    series: BracketSeries,
    load: BracketLoad,
    surface: BracketSurface,
    manufacture_method: BracketManufactureMethod,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum BracketSeries {
    S2020,
    S3030,
    S3060,
    S6060,
    S4040,
    S4080,
    S8080,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum BracketLoad {
    Light(BracketLightLoadSide),
    Standard,
    Strong,
    Heavy,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum BracketLightLoadSide {
    Single,
    Double,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum BracketSurface {
    White,
    Black,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum BracketManufactureMethod {
    Casting,                 // 铸造
    Extrusion(BracketAngle), // 挤压
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum BracketAngle {
    _45,
    _135,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub struct SlotBracketData {
    series: SlotBracketSeries,
    material: SlotBracketMaterial,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum SlotBracketSeries {
    S20,
    S30,
    S40,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum SlotBracketMaterial {
    ZincAlloy, // 锌合金
    Steel,     // 钢
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum ConnectorPlateData {
    Outer(OuterConnectorPlateData),
    Inner(InnerConnectorPlateData),
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub struct OuterConnectorPlateData {
    plate_type: OuterConnectorPlateType,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum OuterConnectorPlateType {
    T,
    L,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub struct InnerConnectorPlateData {
    plate_type: InnerConnectorPlateType,
    hole: Hole,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum InnerConnectorPlateType {
    L,
    Line,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum NutData {
    Normal(NormalNutData),
    Extrude(ExtrudeNutData),
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub struct NormalNutData {
    nut_type: NormalNutType,
    hole: Hole,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum NormalNutType {
    FlangeNut,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub struct ExtrudeNutData {
    extrude_nut_type: ExtrudeNutType,
    series: ExtrudeNutSeries,
    hole: Hole,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum ExtrudeNutSeries {
    SW4mm,
    SW8mm,
    SW10mm,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum ExtrudeNutType {
    Slide,
    T,
    SpringPlate,
    SpringBall,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum Hole {
    M4,
    M5,
    M6,
    M7,
    M8,
    M10,
    M12,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum BoltData {
    Normal(NormalBoltData),
    Extrude(ExtrudeBoltData),
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub struct NormalBoltData {
    bolt_type: NormalBoltType,
    slot_width: u8,
    bolt_length: u16,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum NormalBoltType {
    HalfSphere,
    Plate,
    Cylinder,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub struct ExtrudeBoltData {
    extrude_bolt_type: ExtrudeBoltType,
    hole: Hole,
    bolt_length: u32, // 0.01mm
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum ExtrudeBoltType {
    T,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub struct ElasticFastenerData {
    series: ElasticFastenerSeries,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum ElasticFastenerSeries {
    S30,
    S40,
}
