#![allow(non_snake_case, clippy::empty_docs)]
use nalgebra::Isometry3;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use uuid::Uuid;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    component::{Component, ComponentData, ComponentType, ExtrudeData},
    Quaternion, Translation,
};

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub(crate) id: Uuid,
    pub(crate) component_label: String,
    pub(crate) component_type: ComponentType,
    pub(crate) matrix: Isometry3<f32>,
    pub(crate) config: InstanceConfig,
}

#[wasm_bindgen]
impl Instance {
    pub fn label(&self) -> String {
        self.component_label.clone()
    }

    pub fn component_type(&self) -> ComponentType {
        self.component_type
    }

    pub fn trans(&self) -> Translation {
        Translation {
            x: self.matrix.translation.x,
            y: self.matrix.translation.y,
            z: self.matrix.translation.z,
        }
    }

    pub fn quat(&self) -> Quaternion {
        Quaternion {
            i: self.matrix.rotation.i,
            j: self.matrix.rotation.j,
            k: self.matrix.rotation.k,
            w: self.matrix.rotation.w,
        }
    }

    pub fn instance_config(&self) -> InstanceConfig {
        self.config.clone()
    }
}

impl Instance {
    pub(crate) fn default_component(component: &Component) -> Self {
        let config = match component.data {
            ComponentData::Extrude(_) => InstanceConfig::default_extrude(100000),
            ComponentData::Panel(_) => InstanceConfig::panel(100000, 100000, 2000),
            _ => InstanceConfig::Normal,
        };
        Instance {
            id: Uuid::new_v4(),
            component_label: component.label.clone(),
            component_type: ComponentType::from_data(&component.data),
            matrix: Isometry3::identity(),
            config,
        }
    }

    pub(crate) fn default_extrude(component: &Component, length: u32) -> Option<Instance> {
        match &component.data {
            ComponentData::Extrude(_) => {
                let config = InstanceConfig::default_extrude(length);
                if config.is_extrude_config_valid(component) {
                    Some(Instance {
                        id: Uuid::new_v4(),
                        component_label: component.label.clone(),
                        component_type: ComponentType::from_data(&component.data),
                        matrix: Isometry3::identity(),
                        config: InstanceConfig::default_extrude(length),
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub(crate) fn default_panel(
        component: &Component,
        width: u32,
        height: u32,
        thickness: u32,
    ) -> Option<Instance> {
        match &component.data {
            ComponentData::Panel(_) => Some(Instance {
                id: Uuid::new_v4(),
                component_label: component.label.clone(),
                component_type: ComponentType::from_data(&component.data),
                matrix: Isometry3::identity(),
                config: InstanceConfig::panel(width, height, thickness),
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize, PartialEq, Eq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum InstanceConfig {
    Normal,                 // 默认配置
    Extrude(ExtrudeConfig), // 铝型材
    Panel(PanelConfig),     // 面板，桌板
}

impl InstanceConfig {
    pub fn default_extrude(length: u32) -> Self {
        InstanceConfig::Extrude(ExtrudeConfig {
            drill_left: false,
            drill_right: false,
            bevel_cut: None,
            wrench_hole_left: None,
            wrench_hole_right: None,
            counterbore_left: 0,
            counterbore_right: 0,
            length,
        })
    }

    fn panel(width: u32, height: u32, thickness: u32) -> Self {
        InstanceConfig::Panel(PanelConfig {
            width,
            height,
            thickness,
        })
    }

    pub fn is_extrude_config_valid(&self, component: &Component) -> bool {
        match (self, &component.data) {
            (
                InstanceConfig::Extrude(c),
                ComponentData::Extrude(ExtrudeData { post_process, .. }),
            ) => {
                if !post_process.bevel_cut && c.bevel_cut.is_some() {
                    return false;
                }
                if !post_process.wrench_hole
                    && c.wrench_hole_left.is_some()
                    && c.wrench_hole_right.is_some()
                {
                    return false;
                }
                if !post_process.counterbore && c.counterbore_left > 0 && c.counterbore_right > 0 {
                    return false;
                }
                if c.length < post_process.length.min
                    || c.length > post_process.length.max
                    || c.length % post_process.length.step != 0
                {
                    return false;
                }
                true
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize, PartialEq, Eq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ExtrudeConfig {
    pub drill_left: bool,                      // 左端钻孔
    pub drill_right: bool,                     // 右端钻孔
    pub bevel_cut: Option<BevelCutConfig>,     // 斜切
    pub wrench_hole_left: Option<WrenchHole>,  // 左端扳手孔
    pub wrench_hole_right: Option<WrenchHole>, // 右端扳手孔
    pub counterbore_left: u8,                  // 左端沉头孔 (XA, XB, XC, XD, XE)
    pub counterbore_right: u8,                 // 右端沉头孔 (YA, YB, YC, YD, YE)
    pub length: u32,                           // 长度 精度：0.01mm
}

#[derive(Debug, Clone, Copy, Tsify, Serialize, Deserialize, PartialEq, Eq)]
pub struct WrenchHole {
    pub number: WrenchHoleNumber,
    pub direction: WrenchHoleDirection,
}

#[derive(Debug, Clone, Copy, Tsify, Serialize, Deserialize, PartialEq, Eq)]
pub enum WrenchHoleNumber {
    One,
    Two,
    Three,
}

#[derive(Debug, Clone, Copy, Tsify, Serialize, Deserialize, PartialEq, Eq)]
pub enum WrenchHoleDirection {
    Horizontal, // 水平
    Vertical,   // 垂直
    Both,       // 水平和垂直
}

#[derive(Debug, Clone, Copy, Tsify, Serialize, Deserialize, PartialEq, Eq)]
pub enum BevelCutConfig {
    TopToBottom,     // 断面从上到下
    BottomToTop,     // 断面从下到上
    OutsideToInside, // 断面从外到内
    InsideToOutside, // 断面从内到外
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize, PartialEq, Eq)]
// 0.01mm
pub struct PanelConfig {
    pub width: u32,
    pub height: u32,
    pub thickness: u32,
}

#[cfg(test)]
mod test {
    use crate::component::ComponentLib;

    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn instance_test() {
        let lib = ComponentLib::default();
        lib.components.iter().for_each(|(_label, c)| {
            let _i = Instance::default_component(c);
        });
    }
}
