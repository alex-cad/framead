pub enum ExtrudeConnectorData {
    Bracket(BracketData),                 // 角码
    SlotBracket(SlotBracketData),         // 槽连接件
    ConnectorPlate(ConnectorPlateData),   // 连接板
    Nut(NutData),                         // 螺母
    Bolt(BoltData),                       // 螺栓
    ElasticFastener(ElasticFastenerData), // 弹性扣件
}

pub struct BracketData {
    series: BracketSeries,
    load: BracketLoad,
    surface: BracketSurface,
    manufacture_method: BracketManufactureMethod,
}

pub enum BracketSeries {
    S2020,
    S3030,
    S3060,
    S6060,
    S4040,
    S4080,
    S8080,
}

pub enum BracketLoad {
    Light(BracketLightLoadSide),
    Standard,
    Strong,
    Heavy,
}

pub enum BracketLightLoadSide {
    Single,
    Double,
}

pub enum BracketSurface {
    White,
    Black,
}

pub enum BracketManufactureMethod {
    Casting,                 // 铸造
    Extrusion(BracketAngle), // 挤压
}

pub enum BracketAngle {
    _45,
    _135,
}

pub struct SlotBracketData {
    series: SlotBracketSeries,
    material: SlotBracketMaterial,
}

pub enum SlotBracketSeries {
    S20,
    S30,
    S40,
}

pub enum SlotBracketMaterial {
    ZincAlloy, // 锌合金
    Steel,     // 钢
}

pub enum ConnectorPlateData {
    Outer(OuterConnectorPlateData),
    Inner(InnerConnectorPlateData),
}

pub struct OuterConnectorPlateData {
    plate_type: OuterConnectorPlateType,
}

pub enum OuterConnectorPlateType {
    T,
    L,
}

pub struct InnerConnectorPlateData {
    plate_type: InnerConnectorPlateType,
    hole: Hole,
}

pub enum InnerConnectorPlateType {
    L,
    Line,
}

pub enum NutData {
    Normal(NormalNutData),
    Extrude(ExtrudeNutData),
}

pub struct NormalNutData {
    nut_type: NormalNutType,
    hole: Hole,
}

pub enum NormalNutType {
    FlangeNut,
}

pub struct ExtrudeNutData {
    extrude_nut_type: ExtrudeNutType,
    series: ExtrudeNutSeries,
    hole: Hole,
}

pub enum ExtrudeNutSeries {
    SW4mm,
    SW8mm,
    SW10mm,
}

pub enum ExtrudeNutType {
    Slide,
    T,
    SpringPlate,
    SpringBall,
}

pub enum Hole {
    M4,
    M5,
    M6,
    M7,
    M8,
    M10,
    M12,
}

pub enum BoltData {
    Normal(NormalBoltData),
    Extrude(ExtrudeBoltData),
}

pub struct NormalBoltData {
    bolt_type: NormalBoltType,
    slot_width: u8,
    bolt_length: u16,
}

pub enum NormalBoltType {
    HalfSphere,
    Plate,
    Cylinder,
}

pub struct ExtrudeBoltData {
    extrude_bolt_type: ExtrudeBoltType,
    hole: Hole,
    bolt_length: u16,
}

pub enum ExtrudeBoltType {
    T,
}

pub struct ElasticFastenerData {
    series: ElasticFastenerSeries,
}

pub enum ElasticFastenerSeries {
    S30,
    S40,
}