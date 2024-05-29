#[derive(Debug, Clone)]
pub enum ExtrudeConnectorData {
    Bracket(BracketData),                 // 角码
    SlotBracket(SlotBracketData),         // 槽连接件
    ConnectorPlate(ConnectorPlateData),   // 连接板
    Nut(NutData),                         // 螺母
    Bolt(BoltData),                       // 螺栓
    ElasticFastener(ElasticFastenerData), // 弹性扣件
}

#[derive(Debug, Clone)]
pub struct BracketData {
    series: BracketSeries,
    load: BracketLoad,
    surface: BracketSurface,
    manufacture_method: BracketManufactureMethod,
}

#[derive(Debug, Clone)]
pub enum BracketSeries {
    S2020,
    S3030,
    S3060,
    S6060,
    S4040,
    S4080,
    S8080,
}

#[derive(Debug, Clone)]
pub enum BracketLoad {
    Light(BracketLightLoadSide),
    Standard,
    Strong,
    Heavy,
}

#[derive(Debug, Clone)]
pub enum BracketLightLoadSide {
    Single,
    Double,
}

#[derive(Debug, Clone)]
pub enum BracketSurface {
    White,
    Black,
}

#[derive(Debug, Clone)]
pub enum BracketManufactureMethod {
    Casting,                 // 铸造
    Extrusion(BracketAngle), // 挤压
}

#[derive(Debug, Clone)]
pub enum BracketAngle {
    _45,
    _135,
}

#[derive(Debug, Clone)]
pub struct SlotBracketData {
    series: SlotBracketSeries,
    material: SlotBracketMaterial,
}

#[derive(Debug, Clone)]
pub enum SlotBracketSeries {
    S20,
    S30,
    S40,
}

#[derive(Debug, Clone)]
pub enum SlotBracketMaterial {
    ZincAlloy, // 锌合金
    Steel,     // 钢
}

#[derive(Debug, Clone)]
pub enum ConnectorPlateData {
    Outer(OuterConnectorPlateData),
    Inner(InnerConnectorPlateData),
}

#[derive(Debug, Clone)]
pub struct OuterConnectorPlateData {
    plate_type: OuterConnectorPlateType,
}

#[derive(Debug, Clone)]
pub enum OuterConnectorPlateType {
    T,
    L,
}

#[derive(Debug, Clone)]
pub struct InnerConnectorPlateData {
    plate_type: InnerConnectorPlateType,
    hole: Hole,
}

#[derive(Debug, Clone)]
pub enum InnerConnectorPlateType {
    L,
    Line,
}

#[derive(Debug, Clone)]
pub enum NutData {
    Normal(NormalNutData),
    Extrude(ExtrudeNutData),
}

#[derive(Debug, Clone)]
pub struct NormalNutData {
    nut_type: NormalNutType,
    hole: Hole,
}

#[derive(Debug, Clone)]
pub enum NormalNutType {
    FlangeNut,
}

#[derive(Debug, Clone)]
pub struct ExtrudeNutData {
    extrude_nut_type: ExtrudeNutType,
    series: ExtrudeNutSeries,
    hole: Hole,
}

#[derive(Debug, Clone)]
pub enum ExtrudeNutSeries {
    SW4mm,
    SW8mm,
    SW10mm,
}

#[derive(Debug, Clone)]
pub enum ExtrudeNutType {
    Slide,
    T,
    SpringPlate,
    SpringBall,
}

#[derive(Debug, Clone)]
pub enum Hole {
    M4,
    M5,
    M6,
    M7,
    M8,
    M10,
    M12,
}

#[derive(Debug, Clone)]
pub enum BoltData {
    Normal(NormalBoltData),
    Extrude(ExtrudeBoltData),
}

#[derive(Debug, Clone)]
pub struct NormalBoltData {
    bolt_type: NormalBoltType,
    slot_width: u8,
    bolt_length: u16,
}

#[derive(Debug, Clone)]
pub enum NormalBoltType {
    HalfSphere,
    Plate,
    Cylinder,
}

#[derive(Debug, Clone)]
pub struct ExtrudeBoltData {
    extrude_bolt_type: ExtrudeBoltType,
    hole: Hole,
    bolt_length: u16,
}

#[derive(Debug, Clone)]
pub enum ExtrudeBoltType {
    T,
}

#[derive(Debug, Clone)]
pub struct ElasticFastenerData {
    series: ElasticFastenerSeries,
}

#[derive(Debug, Clone)]
pub enum ElasticFastenerSeries {
    S30,
    S40,
}
