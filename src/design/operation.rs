use nalgebra::Isometry3;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use uuid::Uuid;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    component::Component,
    design::DesignSpace,
    instance::{ExtrudeConfig, Instance, InstanceConfig},
    Quaternion, Translation,
};

/// an edit operation that can be applied to a target
pub trait Operation {
    type Target;
    fn operate(&mut self, target: &mut Self::Target);
    fn inverse(&mut self, target: &mut Self::Target);
    fn compress(&mut self, target: &Self) -> bool;
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct AddInstance {
    instance: Instance,
}

impl AddInstance {
    pub fn default_component(component: &Component) -> Self {
        AddInstance {
            instance: Instance::default_component(component),
        }
    }

    pub fn extrude(component: &Component, length: u32) -> Option<Self> {
        Some(AddInstance {
            instance: Instance::default_extrude(component, length)?,
        })
    }

    pub fn panel(component: &Component, width: u32, height: u32, thickness: u32) -> Option<Self> {
        Some(AddInstance {
            instance: Instance::default_panel(component, width, height, thickness)?,
        })
    }
}

impl Operation for AddInstance {
    type Target = DesignSpace;

    fn operate(&mut self, target: &mut Self::Target) {
        target.instances.push(self.instance.clone());
    }

    fn inverse(&mut self, target: &mut Self::Target) {
        target.instances.retain(|i| i.id != self.instance.id);
    }

    fn compress(&mut self, _target: &Self) -> bool {
        false
    }
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveInstance {
    pub(crate) id: Uuid,
    pub(crate) removed_instance: Option<Instance>,
}

impl Operation for RemoveInstance {
    type Target = DesignSpace;

    fn operate(&mut self, target: &mut Self::Target) {
        // find the instance by id,
        // save it to cache, and remove it from instances
        let index = target.instances.iter().position(|i| i.id == self.id);
        if let Some(index) = index {
            let instance = target.instances.remove(index);
            self.removed_instance.replace(instance);
        }
    }

    fn inverse(&mut self, target: &mut Self::Target) {
        if let Some(instance) = self.removed_instance.take() {
            target.instances.push(instance);
        }
    }

    fn compress(&mut self, _target: &Self) -> bool {
        false
    }
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct PostProcessInstance {
    pub(crate) id: Uuid,
    pub(crate) config: InstanceConfig,
    pub(crate) config_cache: Option<InstanceConfig>,
}

impl Operation for PostProcessInstance {
    type Target = DesignSpace;

    fn operate(&mut self, target: &mut Self::Target) {
        let index = target.instances.iter().position(|i| i.id == self.id);
        if let Some(index) = index {
            let config = match (self.config.clone(), &target.instances[index].config) {
                (InstanceConfig::Extrude(mut e), InstanceConfig::Extrude(target_e)) => {
                    e.length = target_e.length;
                    InstanceConfig::Extrude(e)
                }
                (config, _) => config,
            };
            // cache the old config
            self.config_cache
                .replace(target.instances[index].config.clone());
            target.instances[index].config = config;
        }
    }

    fn inverse(&mut self, target: &mut Self::Target) {
        let index = target.instances.iter().position(|i| i.id == self.id);
        if let Some(index) = index {
            target.instances[index].config = self.config_cache.take().unwrap();
        }
    }

    fn compress(&mut self, _target: &Self) -> bool {
        false
    }
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrudeLength {
    pub(crate) id: Uuid,
    pub(crate) dlength: i32,
    pub(crate) matrix: Isometry3<f32>,
}

impl Operation for ExtrudeLength {
    type Target = DesignSpace;

    fn operate(&mut self, target: &mut Self::Target) {
        let index = target.instances.iter().position(|i| i.id == self.id);
        if let Some(index) = index {
            if let InstanceConfig::Extrude(e) = &mut target.instances[index].config {
                e.length = ((e.length as i32) + self.dlength) as u32;
                target.instances[index].matrix *= self.matrix;
            }
        }
    }

    fn inverse(&mut self, target: &mut Self::Target) {
        let index = target.instances.iter().position(|i| i.id == self.id);
        if let Some(index) = index {
            if let InstanceConfig::Extrude(e) = &mut target.instances[index].config {
                e.length = ((e.length as i32) - self.dlength) as u32;
                target.instances[index].matrix *= self.matrix.inverse();
            }
        }
    }

    fn compress(&mut self, target: &Self) -> bool {
        self.dlength += target.dlength;
        true
    }
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct PanelSize {
    pub(crate) id: Uuid,
    pub(crate) dwidth: i32,
    pub(crate) dheight: i32,
    pub(crate) dthickness: i32,
    pub(crate) matrix: Isometry3<f32>,
}

impl Operation for PanelSize {
    type Target = DesignSpace;
    fn operate(&mut self, target: &mut Self::Target) {
        let index = target.instances.iter().position(|i| i.id == self.id);
        if let Some(index) = index {
            if let InstanceConfig::Panel(p) = &mut target.instances[index].config {
                p.width = ((p.width as i32) + self.dwidth) as u32;
                p.height = ((p.height as i32) + self.dheight) as u32;
                p.thickness = ((p.thickness as i32) + self.dthickness) as u32;
                target.instances[index].matrix *= self.matrix;
            }
        }
    }

    fn inverse(&mut self, target: &mut Self::Target) {
        let index = target.instances.iter().position(|i| i.id == self.id);
        if let Some(index) = index {
            if let InstanceConfig::Panel(p) = &mut target.instances[index].config {
                p.width = ((p.width as i32) - self.dwidth) as u32;
                p.height = ((p.height as i32) - self.dheight) as u32;
                p.thickness = ((p.thickness as i32) - self.dthickness) as u32;
                target.instances[index].matrix *= self.matrix.inverse();
            }
        }
    }

    fn compress(&mut self, target: &Self) -> bool {
        self.dwidth += target.dwidth;
        self.dheight += target.dheight;
        self.dthickness += target.dthickness;
        true
    }
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct MoveInstance {
    pub(crate) id: Uuid,
    pub(crate) matrix: Isometry3<f32>,
}

impl Operation for MoveInstance {
    type Target = DesignSpace;

    fn operate(&mut self, target: &mut Self::Target) {
        let index = target.instances.iter().position(|i| i.id == self.id);
        if let Some(index) = index {
            target.instances[index].matrix *= self.matrix;
        }
    }

    fn inverse(&mut self, target: &mut Self::Target) {
        let index = target.instances.iter().position(|i| i.id == self.id);
        if let Some(index) = index {
            target.instances[index].matrix *= self.matrix.inverse();
        }
    }

    fn compress(&mut self, target: &Self) -> bool {
        self.matrix *= target.matrix;
        true
    }
}

#[allow(non_snake_case, clippy::empty_docs)]
pub mod allow_non_snake_case {
    use super::*;
    #[derive(Debug, Tsify, Serialize, Deserialize)]
    #[tsify(into_wasm_abi, from_wasm_abi)]
    pub enum DesignOperation {
        AddInstance(AddInstance),
        RemoveInstance(RemoveInstance),
        PostProcessInstance(PostProcessInstance),
        ExtrudeLength(ExtrudeLength),
        PanelSize(PanelSize),
        MoveInstance(MoveInstance),
        // AddConstraint,
        // RemoveConstraint,
        // ConfigConstraint,
        // AddInput,
        // RemoveInput,
        // ConfigInput,
    }
}

pub use allow_non_snake_case::DesignOperation;

impl Operation for DesignOperation {
    type Target = DesignSpace;

    fn operate(&mut self, target: &mut Self::Target) {
        match self {
            DesignOperation::AddInstance(op) => op.operate(target),
            DesignOperation::RemoveInstance(op) => op.operate(target),
            DesignOperation::PostProcessInstance(op) => op.operate(target),
            DesignOperation::ExtrudeLength(op) => op.operate(target),
            DesignOperation::PanelSize(op) => op.operate(target),
            DesignOperation::MoveInstance(op) => op.operate(target),
        }
    }

    fn inverse(&mut self, target: &mut Self::Target) {
        match self {
            DesignOperation::AddInstance(op) => op.inverse(target),
            DesignOperation::RemoveInstance(op) => op.inverse(target),
            DesignOperation::PostProcessInstance(op) => op.inverse(target),
            DesignOperation::ExtrudeLength(op) => op.inverse(target),
            DesignOperation::PanelSize(op) => op.inverse(target),
            DesignOperation::MoveInstance(op) => op.inverse(target),
        }
    }

    fn compress(&mut self, target: &Self) -> bool {
        match (self, target) {
            (DesignOperation::ExtrudeLength(op), DesignOperation::ExtrudeLength(target)) => {
                op.compress(target)
            }
            (DesignOperation::MoveInstance(op), DesignOperation::MoveInstance(target)) => {
                op.compress(target)
            }
            (DesignOperation::PanelSize(op), DesignOperation::PanelSize(target)) => {
                op.compress(target)
            }
            _ => false,
        }
    }
}

use crate::component::ComponentData::*;

#[wasm_bindgen]
pub fn add_normal_instance(component: &Component) -> Result<DesignOperation, String> {
    match component.data {
        Extrude(_) | Panel(_) => Err("invalid component type: Extrude or Panel".to_string()),
        _ => Ok(DesignOperation::AddInstance(
            AddInstance::default_component(component),
        )),
    }
}

#[wasm_bindgen]
pub fn add_extrude_instance(component: &Component, length: u32) -> Result<DesignOperation, String> {
    if let Some(op) = AddInstance::extrude(component, length) {
        Ok(DesignOperation::AddInstance(op))
    } else {
        Err("invalid component type: Not Extrude or Bad Length".to_string())
    }
}

#[wasm_bindgen]
pub fn add_panel_instance(
    component: &Component,
    width: u32,
    height: u32,
    thickness: u32,
) -> Result<DesignOperation, String> {
    if let Some(op) = AddInstance::panel(component, width, height, thickness) {
        Ok(DesignOperation::AddInstance(op))
    } else {
        Err("invalid component type: Not Extrude".to_string())
    }
}

#[wasm_bindgen]
pub fn remove_instance(instance: &Instance) -> DesignOperation {
    DesignOperation::RemoveInstance(RemoveInstance {
        id: instance.id,
        removed_instance: None,
    })
}

#[wasm_bindgen]
pub fn extrude_post_process(
    instance: &Instance,
    component: &Component,
    config: ExtrudeConfig,
) -> Result<DesignOperation, String> {
    let config = InstanceConfig::Extrude(config);
    if config.is_extrude_config_valid(component) {
        Ok(DesignOperation::PostProcessInstance(PostProcessInstance {
            id: instance.id,
            config,
            config_cache: None,
        }))
    } else {
        Err("invalid config".into())
    }
}

#[wasm_bindgen]
pub fn extrude_add_length(
    instance: &Instance,
    d_length: i32,
    tra: Translation,
    quat: Quaternion,
) -> DesignOperation {
    DesignOperation::ExtrudeLength(ExtrudeLength {
        id: instance.id,
        dlength: d_length,
        matrix: nalgebra::Isometry3::from_parts(
            nalgebra::Translation3::new(tra.x, tra.y, tra.z),
            nalgebra::UnitQuaternion::new_normalize(nalgebra::Quaternion::new(
                quat.w, quat.i, quat.j, quat.k,
            )),
        ),
    })
}

#[wasm_bindgen]
pub fn panel_add_size(
    instance: &Instance,
    dwidth: i32,
    dheight: i32,
    dthickness: i32,
    tra: Translation,
    quat: Quaternion,
) -> DesignOperation {
    DesignOperation::PanelSize(PanelSize {
        id: instance.id,
        dwidth,
        dheight,
        dthickness,
        matrix: nalgebra::Isometry3::from_parts(
            nalgebra::Translation3::new(tra.x, tra.y, tra.z),
            nalgebra::UnitQuaternion::new_normalize(nalgebra::Quaternion::new(
                quat.w, quat.i, quat.j, quat.k,
            )),
        ),
    })
}

#[wasm_bindgen]
pub fn move_instance(instance: &Instance, tra: Translation, quat: Quaternion) -> DesignOperation {
    DesignOperation::MoveInstance(MoveInstance {
        id: instance.id,
        matrix: nalgebra::Isometry3::from_parts(
            nalgebra::Translation3::new(tra.x, tra.y, tra.z),
            nalgebra::UnitQuaternion::new_normalize(nalgebra::Quaternion::new(
                quat.w, quat.i, quat.j, quat.k,
            )),
        ),
    })
}

#[cfg(test)]
mod test {
    use crate::{
        component::{ComponentLib, ComponentType},
        instance::PanelConfig,
    };

    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_add_normal_instance() {
        let lib = ComponentLib::default();
        let component = lib.components.get("Accessory-test").unwrap();
        let op = add_normal_instance(component).unwrap();
        if let DesignOperation::AddInstance(op) = op {
            assert_ne!(op.instance.id, Uuid::new_v4());
            assert_eq!(op.instance.component_label, "Accessory-test");
            assert_eq!(op.instance.component_type, ComponentType::Accessory);
            assert_eq!(op.instance.config, InstanceConfig::Normal);
            assert_eq!(op.instance.matrix, Isometry3::identity());
        } else {
            panic!("invalid operation type");
        }
    }

    #[wasm_bindgen_test]
    fn test_add_extrude_instance() {
        let lib = ComponentLib::default();
        let component = lib.components.get("LCF8-4040").unwrap();
        let op = add_extrude_instance(component, 100);
        assert!(op.is_err());
        let op = add_extrude_instance(component, 10000).unwrap();
        if let DesignOperation::AddInstance(op) = op {
            assert_ne!(op.instance.id, Uuid::new_v4());
            assert_eq!(op.instance.component_label, "LCF8-4040");
            assert_eq!(op.instance.component_type, ComponentType::Extrude);
            assert_eq!(
                op.instance.config,
                InstanceConfig::Extrude(ExtrudeConfig {
                    drill_left: false,
                    drill_right: false,
                    bevel_cut: None,
                    wrench_hole_left: None,
                    wrench_hole_right: None,
                    counterbore_left: 0,
                    counterbore_right: 0,
                    length: 10000,
                })
            );
            assert_eq!(op.instance.matrix, Isometry3::identity());
        } else {
            panic!("invalid operation type");
        }
    }

    #[wasm_bindgen_test]
    fn test_add_panel_instance() {
        let lib = ComponentLib::default();
        let component = lib.components.get("WoodenPanel-test").unwrap();
        let op = add_panel_instance(component, 100000, 100000, 2000).unwrap();
        if let DesignOperation::AddInstance(op) = op {
            assert_ne!(op.instance.id, Uuid::new_v4());
            assert_eq!(op.instance.component_label, "WoodenPanel-test");
            assert_eq!(op.instance.component_type, ComponentType::Panel);
            assert_eq!(
                op.instance.config,
                InstanceConfig::Panel(PanelConfig {
                    width: 100000,
                    height: 100000,
                    thickness: 2000,
                })
            );
            assert_eq!(op.instance.matrix, Isometry3::identity());
        } else {
            panic!("invalid operation type");
        }
    }

    #[wasm_bindgen_test]
    fn test_remove_instance() {
        let instance = Instance {
            id: Uuid::new_v4(),
            component_label: "test".to_string(),
            component_type: ComponentType::Accessory,
            matrix: Isometry3::identity(),
            config: InstanceConfig::Normal,
        };
        let op = remove_instance(&instance);
        if let DesignOperation::RemoveInstance(op) = op {
            assert_eq!(op.id, instance.id);
            assert!(op.removed_instance.is_none());
        } else {
            panic!("invalid operation type");
        }
    }

    #[wasm_bindgen_test]
    fn test_extrude_post_process() {
        let lib = ComponentLib::default();
        let component = lib.components.get("LCF8-4040").unwrap();
        let instance = Instance::default_extrude(component, 10000).unwrap();
        let config = ExtrudeConfig {
            drill_left: true,
            drill_right: true,
            bevel_cut: None,
            wrench_hole_left: None,
            wrench_hole_right: None,
            counterbore_left: 5,
            counterbore_right: 5,
            length: 10000,
        };
        let op = extrude_post_process(&instance, component, config.clone()).unwrap();
        if let DesignOperation::PostProcessInstance(op) = op {
            assert_eq!(op.id, instance.id);
            assert_eq!(op.config, InstanceConfig::Extrude(config.clone()));
            assert_eq!(op.config_cache, None);
        } else {
            panic!("invalid operation type");
        }
    }

    #[wasm_bindgen_test]
    fn test_extrude_add_length() {
        let instance = Instance {
            id: Uuid::new_v4(),
            component_label: "test".to_string(),
            component_type: ComponentType::Extrude,
            matrix: Isometry3::identity(),
            config: InstanceConfig::Extrude(ExtrudeConfig {
                drill_left: false,
                drill_right: false,
                bevel_cut: None,
                wrench_hole_left: None,
                wrench_hole_right: None,
                counterbore_left: 0,
                counterbore_right: 0,
                length: 10000,
            }),
        };
        let op = extrude_add_length(
            &instance,
            1000,
            Translation::identity(),
            Quaternion::identity(),
        );
        if let DesignOperation::ExtrudeLength(op) = op {
            assert_eq!(op.id, instance.id);
            assert_eq!(op.dlength, 1000);
            assert_eq!(op.matrix, Isometry3::identity());
        } else {
            panic!("invalid operation type");
        }
    }

    #[wasm_bindgen_test]
    fn test_panel_add_size() {
        let instance = Instance {
            id: Uuid::new_v4(),
            component_label: "test".to_string(),
            component_type: ComponentType::Panel,
            matrix: Isometry3::identity(),
            config: InstanceConfig::Panel(PanelConfig {
                width: 1000,
                height: 1000,
                thickness: 100,
            }),
        };
        let op = panel_add_size(
            &instance,
            100,
            100,
            10,
            Translation::identity(),
            Quaternion::identity(),
        );
        if let DesignOperation::PanelSize(op) = op {
            assert_eq!(op.id, instance.id);
            assert_eq!(op.dwidth, 100);
            assert_eq!(op.dheight, 100);
            assert_eq!(op.dthickness, 10);
            assert_eq!(op.matrix, Isometry3::identity());
        } else {
            panic!("invalid operation type");
        }
    }

    #[wasm_bindgen_test]
    fn test_move_instance() {
        let instance = Instance {
            id: Uuid::new_v4(),
            component_label: "test".to_string(),
            component_type: ComponentType::Panel,
            matrix: Isometry3::identity(),
            config: InstanceConfig::Panel(PanelConfig {
                width: 1000,
                height: 1000,
                thickness: 100,
            }),
        };
        let op = move_instance(
            &instance,
            Translation::identity(),
            Quaternion::identity(),
        );
        if let DesignOperation::MoveInstance(op) = op {
            assert_eq!(op.id, instance.id);
            assert_eq!(op.matrix, Isometry3::identity());
        } else {
            panic!("invalid operation type");
        }
    }
}
