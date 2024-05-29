use nalgebra::Matrix4;
use uuid::Uuid;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::component::{Component, ComponentData, ExtrudeData};

#[derive(Debug, Clone)]
pub struct Instance {
    pub(crate) id: Uuid,
    component_label: String,
    pub(crate) matrix: Matrix4<f32>,
    pub(crate) config: InstanceConfig,
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
            matrix: Matrix4::identity(),
            config,
        }
    }

    pub(crate) fn default_extrude(component: &Component, length: u32) -> Option<Instance> {
        match &component.data {
            ComponentData::Extrude(_) => Some(Instance {
                id: Uuid::new_v4(),
                component_label: component.label.clone(),
                matrix: Matrix4::identity(),
                config: InstanceConfig::default_extrude(length),
            }),
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
                matrix: Matrix4::identity(),
                config: InstanceConfig::panel(width, height, thickness),
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
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

    pub fn is_valid(&self, component: &Component) -> bool {
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
            (InstanceConfig::Panel(_), ComponentData::Panel(_)) => true,
            (InstanceConfig::Normal, _) => true,
            _ => false,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
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

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct WrenchHole {
    pub number: WrenchHoleNumber,
    pub direction: WrenchHoleDirection,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum WrenchHoleNumber {
    One,
    Two,
    Three,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum WrenchHoleDirection {
    Horizontal, // 水平
    Vertical,   // 垂直
    Both,       // 水平和垂直
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum BevelCutConfig {
    TopToBottom,     // 断面从上到下
    BottomToTop,     // 断面从下到上
    OutsideToInside, // 断面从外到内
    InsideToOutside, // 断面从内到外
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
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
