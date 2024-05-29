mod end_cap;
mod extrude;
mod extrude_connector;
mod floor;
mod panel;
use std::collections::HashMap;

pub use end_cap::EndCapData;
pub use extrude::ExtrudeData;
pub use extrude_connector::ExtrudeConnectorData;
pub use floor::FloorData;
pub use panel::PanelData;
use wasm_bindgen::prelude::wasm_bindgen;

use self::extrude::{
    CounterboreSize, Drill, ExtrudeLength, ExtrudePostProcess, ExtrudeRectShape, ExtrudeSeries,
    ExtrudeShape, ExtrudeShapeEnum, ExtrudeSquareShape, ExtrudeStandard, ExtrudeSurface, Metarial,
    S40ExtrudeSlotDepth,
};

#[derive(Debug, Clone)]
pub enum ComponentData {
    Extrude(ExtrudeData),                   // 铝型材
    ExtrudeConnector(ExtrudeConnectorData), // 铝型材连接器，包括角码、连接板、螺母、螺栓等
    Floor(FloorData),                       // 地脚, 轮子
    Door,                                   // 门
    Panel(PanelData),                       // 面板，桌板
    EndCap(EndCapData),                     // 端盖
    SlotCover,                              // 槽盖
    Accessory,                              // 配件
}

#[derive(Debug, Clone)]
pub struct Vender {
    name: String,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Component {
    pub(crate) label: String,
    name: String,
    pub(crate) data: ComponentData,
    vendor: Vender,
}

impl Component {
    pub fn new(label: String, name: String, data: ComponentData, vendor: Vender) -> Self {
        Component {
            label,
            name,
            data,
            vendor,
        }
    }
}

// #[wasm_bindgen]
pub struct ComponentLib {
    pub(crate) components: HashMap<String, Component>,
}

// #[wasm_bindgen]
impl ComponentLib {
    // #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        ComponentLib {
            components: HashMap::new(),
        }
    }

    pub fn add_component(&mut self, component: Component) {
        self.components.insert(component.label.clone(), component);
    }

    pub fn get_component(&self, label: String) -> Option<Component> {
        self.components.get(&label).cloned()
    }
}

impl Default for ComponentLib {
    fn default() -> Self {
        let mut lib = ComponentLib::new();
        let misumi = Vender {
            name: "Misumi".to_string(),
        };
        lib.add_component(Component {
            label: "LCF8-4040".into(),
            name: "4040 欧标铝型材".into(),
            data: ComponentData::Extrude(ExtrudeData {
                standard: ExtrudeStandard {
                    series: ExtrudeSeries::S40(S40ExtrudeSlotDepth::SlotDepth12_3mm),
                    metarial: Metarial::_6063T5,
                    surface: ExtrudeSurface::AA10,
                },
                shape: ExtrudeShape {
                    name: "LCF8-4040".into(),
                    shape: ExtrudeShapeEnum::Square(ExtrudeSquareShape::FourSlot),
                    holes_count: 1,
                },
                post_process: ExtrudePostProcess {
                    drill: Drill::M8_25mm,
                    bevel_cut: true,
                    wrench_hole: true,
                    wrench_hole_size: 7,
                    counterbore: true,
                    counterbore_size: CounterboreSize::Z8,
                    length: ExtrudeLength {
                        min: 5000,
                        max: 400000,
                        step: 50,
                    },
                },
            }),
            vendor: misumi.clone(),
        });

        lib.add_component(Component {
            label: "LCF8-4080".into(),
            name: "4080 欧标铝型材".into(),
            data: ComponentData::Extrude(ExtrudeData {
                standard: ExtrudeStandard {
                    series: ExtrudeSeries::S40(S40ExtrudeSlotDepth::SlotDepth12_3mm),
                    metarial: Metarial::_6063T5,
                    surface: ExtrudeSurface::AA10,
                },
                shape: ExtrudeShape {
                    name: "LCF8-4080".into(),
                    shape: ExtrudeShapeEnum::Rect(2, ExtrudeRectShape::FourSlot),
                    holes_count: 2,
                },
                post_process: ExtrudePostProcess {
                    drill: Drill::M8_25mm,
                    bevel_cut: true,
                    wrench_hole: true,
                    wrench_hole_size: 7,
                    counterbore: true,
                    counterbore_size: CounterboreSize::Z8,
                    length: ExtrudeLength {
                        min: 5000,
                        max: 400000,
                        step: 50,
                    },
                },
            }),
            vendor: misumi.clone(),
        });

        lib.add_component(Component {
            label: "LCF8-40160".into(),
            name: "40160 欧标铝型材".into(),
            data: ComponentData::Extrude(ExtrudeData {
                standard: ExtrudeStandard {
                    series: ExtrudeSeries::S40(S40ExtrudeSlotDepth::SlotDepth12_3mm),
                    metarial: Metarial::_6063T5,
                    surface: ExtrudeSurface::AA10,
                },
                shape: ExtrudeShape {
                    name: "LCF8-40160".into(),
                    shape: ExtrudeShapeEnum::Rect(4, ExtrudeRectShape::FourSlot),
                    holes_count: 4,
                },
                post_process: ExtrudePostProcess {
                    drill: Drill::M8_25mm,
                    bevel_cut: true,
                    wrench_hole: true,
                    wrench_hole_size: 7,
                    counterbore: true,
                    counterbore_size: CounterboreSize::Z8,
                    length: ExtrudeLength {
                        min: 5000,
                        max: 400000,
                        step: 50,
                    },
                },
            }),
            vendor: misumi.clone(),
        });

        lib.add_component(Component {
            label: "WoodenPanel-test".into(),
            name: "WoodenPanel-test".into(),
            data: ComponentData::Panel(PanelData::Wood),
            vendor: Vender {
                name: "Fake Panel Maker".to_string(),
            },
        });
        lib
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn component_lib_test() {
        let lib = ComponentLib::default();
        assert_eq!(lib.components.len(), 4);
    }
}
