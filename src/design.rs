mod operation;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::instance::Instance;

use operation::{DesignOperation, Operation};

trait Record {
    type Operation;
    fn push(&mut self, edit: Self::Operation);

    // ctrl+z
    fn pop(&mut self) -> Option<&Self::Operation>;

    // shift+ctrl+z
    fn repush(&mut self);
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct DesignSpace {
    instances: Vec<Instance>,
    // constraints: Vec<ConstraintSystem>,
    records: Vec<DesignOperation>,

    poped: Vec<DesignOperation>,
}

#[wasm_bindgen]
impl DesignSpace {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        DesignSpace {
            instances: Vec::new(),
            records: Vec::new(),
            poped: Vec::new(),
        }
    }

    pub fn get_instances(&self) -> Vec<Instance> {
        self.instances.clone()
    }
}

impl Default for DesignSpace {
    fn default() -> Self {
        DesignSpace::new()
    }
}

impl Record for DesignSpace {
    type Operation = DesignOperation;

    fn push(&mut self, mut edit: Self::Operation) {
        edit.operate(self);
        if self.records.is_empty() || !self.records.last_mut().unwrap().compress(&edit) {
            self.records.push(edit);
            self.poped.clear();
        }
    }

    fn pop(&mut self) -> Option<&Self::Operation> {
        if let Some(mut o) = self.records.pop() {
            o.inverse(self);
            self.poped.push(o);
            self.poped.last()
        } else {
            None
        }
    }

    fn repush(&mut self) {
        if let Some(mut o) = self.poped.pop() {
            o.operate(self);
            if self.records.is_empty() || !self.records.last_mut().unwrap().compress(&o) {
                self.records.push(o);
            }
        }
    }
}

#[wasm_bindgen]
impl DesignSpace {
    pub fn push(&mut self, edit: DesignOperation) {
        <Self as Record>::push(self, edit);
    }
    // pub fn add_normal_instance(&mut self, component: &Component) -> Result<(), String> {
    //     use crate::component::ComponentData::*;
    //     match component.data {
    //         Extrude(_) | Panel(_) => {
    //             return Err("invalid component type".to_string());
    //         }
    //         _ => {}
    //     }
    //     let op = AddInstance::default_component(component);
    //     <Self as Record>::push(self, DesignOperation::AddInstance(op));
    //     Ok(())
    // }

    // pub fn add_extrude_instance(
    //     &mut self,
    //     component: &Component,
    //     length: u32,
    // ) -> Result<(), String> {
    //     use crate::component::ComponentData::*;
    //     match component.data {
    //         Extrude(_) => {}
    //         _ => {
    //             return Err("invalid component type".to_string());
    //         }
    //     }
    //     let op = AddInstance::extrude(component, length).unwrap();
    //     <Self as Record>::push(self, DesignOperation::AddInstance(op));
    //     Ok(())
    // }

    // pub fn add_panel_instance(
    //     &mut self,
    //     component: &Component,
    //     width: u32,
    //     height: u32,
    //     thickness: u32,
    // ) -> Result<(), String> {
    //     use crate::component::ComponentData::*;
    //     match component.data {
    //         Panel(_) => {}
    //         _ => {
    //             return Err("invalid component type".to_string());
    //         }
    //     }
    //     let op = AddInstance::panel(component, width, height, thickness).unwrap();
    //     <Self as Record>::push(self, DesignOperation::AddInstance(op));
    //     Ok(())
    // }

    // pub fn remove_instance(&mut self, id: &str) -> Result<(), String> {
    //     let op = operation::RemoveInstance {
    //         id: Uuid::parse_str(id).map_err(|e| e.to_string())?,
    //         removed_instance: None,
    //     };
    //     <Self as Record>::push(self, DesignOperation::RemoveInstance(op));
    //     Ok(())
    // }

    // pub fn extrude_post_process(&mut self, id: &str, config: ExtrudeConfig) -> Result<(), String> {
    //     let op = operation::PostProcessInstance {
    //         id: Uuid::parse_str(id).map_err(|e| e.to_string())?,
    //         config: InstanceConfig::Extrude(config),
    //         config_cache: None,
    //     };
    //     <Self as Record>::push(self, DesignOperation::PostProcessInstance(op));
    //     Ok(())
    // }

    // pub fn extrude_add_length(&mut self, id: &str, dlength: i32) -> Result<(), String> {
    //     let op = operation::ProfileLength {
    //         id: Uuid::parse_str(id).map_err(|e| e.to_string())?,
    //         dlength,
    //     };
    //     <Self as Record>::push(self, DesignOperation::ProfileLength(op));
    //     Ok(())
    // }

    // pub fn panel_add_size(
    //     &mut self,
    //     id: &str,
    //     dwidth: i32,
    //     dheight: i32,
    //     dthickness: i32,
    // ) -> Result<(), String> {
    //     let op = operation::PanelAddSize {
    //         id: Uuid::parse_str(id).map_err(|e| e.to_string())?,
    //         dwidth,
    //         dheight,
    //         dthickness,
    //     };
    //     <Self as Record>::push(self, DesignOperation::PanelAddSize(op));
    //     Ok(())
    // }

    // pub fn move_instance(
    //     &mut self,
    //     id: &str,
    //     tra: InstanceTrans,
    //     euler_angles: EulerAngles,
    // ) -> Result<(), String> {
    //     let op = operation::MoveInstance {
    //         id: Uuid::parse_str(id).map_err(|e| e.to_string())?,
    //         matrix: nalgebra::Isometry3::from_parts(
    //             Translation3::new(tra.x, tra.y, tra.z),
    //             UnitQuaternion::from_euler_angles(
    //                 euler_angles.roll,
    //                 euler_angles.pitch,
    //                 euler_angles.yaw,
    //             ),
    //         ),
    //     };
    //     <Self as Record>::push(self, DesignOperation::MoveInstance(op));
    //     Ok(())
    // }

    pub fn pop(&mut self) {
        <Self as Record>::pop(self);
    }

    pub fn repush(&mut self) {
        <Self as Record>::repush(self);
    }
}

#[cfg(test)]
mod test {
    use crate::{
        component::ComponentLib,
        instance::{ExtrudeConfig, InstanceConfig}, log,
        // log,
    };

    use super::*;
    use nalgebra::Isometry3;
    use test::operation::*;
    use wasm_bindgen_test::wasm_bindgen_test;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn add_instance_operate_test() {
        let lib = ComponentLib::default();
        let mut design = DesignSpace::new();

        // let add = AddInstance::default_component(
        //     lib.components.get("LCF8-4040").unwrap(),
        //     InstanceConfig::default_extrude(100000), // 1m
        // )
        // .unwrap();
        let add = AddInstance::extrude(lib.components.get("LCF8-4040").unwrap(), 100000).unwrap();
        design.push(DesignOperation::AddInstance(add));
        assert_eq!(design.instances.len(), 1);
        assert_eq!(design.records.len(), 1);
        // log(&format!("{:#?}", design));
    }

    #[wasm_bindgen_test]
    fn add_instance_inv_operate_test() {
        let lib = ComponentLib::default();
        let mut design = DesignSpace::new();

        let add = AddInstance::extrude(lib.components.get("LCF8-4040").unwrap(), 100000).unwrap();
        design.push(DesignOperation::AddInstance(add));
        design.pop();
        assert_eq!(design.instances.len(), 0);
        assert_eq!(design.records.len(), 0);
        assert_eq!(design.poped.len(), 1);
        // log(&format!("{:#?}", design));
        design.repush();
        assert_eq!(design.instances.len(), 1);
        assert_eq!(design.records.len(), 1);
        assert_eq!(design.poped.len(), 0);
        // log(&format!("{:#?}", design));
    }

    #[wasm_bindgen_test]
    fn remove_instance_operate_test() {
        let lib = ComponentLib::default();
        let mut design = DesignSpace::new();

        let add = AddInstance::extrude(lib.components.get("LCF8-4040").unwrap(), 100000).unwrap();
        design.push(DesignOperation::AddInstance(add));
        let id = design.instances[0].id;

        let remove = RemoveInstance {
            id,
            removed_instance: None,
        };
        design.push(DesignOperation::RemoveInstance(remove));
        assert_eq!(design.instances.len(), 0);
        assert_eq!(design.records.len(), 2);
        // log(&format!("{:#?}", design));
    }

    #[wasm_bindgen_test]
    fn remove_instance_inv_operate_test() {
        let lib = ComponentLib::default();
        let mut design = DesignSpace::new();

        let add = AddInstance::extrude(lib.components.get("LCF8-4040").unwrap(), 100000).unwrap();
        design.push(DesignOperation::AddInstance(add));
        let id = design.instances[0].id;

        let remove = RemoveInstance {
            id,
            removed_instance: None,
        };
        design.push(DesignOperation::RemoveInstance(remove));
        assert_eq!(design.instances.len(), 0);
        assert_eq!(design.records.len(), 2);
        design.pop();
        assert_eq!(design.instances.len(), 1);
        assert_eq!(design.records.len(), 1);
        // log(&format!("{:#?}", design));
        design.repush();
        assert_eq!(design.instances.len(), 0);
        assert_eq!(design.records.len(), 2);
        // log(&format!("{:#?}", design));
    }

    #[wasm_bindgen_test]
    fn config_instance_operate_test() {
        let lib = ComponentLib::default();
        let mut design = DesignSpace::new();

        let add = AddInstance::extrude(lib.components.get("LCF8-4040").unwrap(), 100000).unwrap();
        design.push(DesignOperation::AddInstance(add));
        design.push(DesignOperation::PostProcessInstance(PostProcessInstance {
            id: design.instances[0].id,
            config: InstanceConfig::Extrude(ExtrudeConfig {
                drill_left: true,
                drill_right: false,
                bevel_cut: None,
                wrench_hole_left: None,
                wrench_hole_right: None,
                counterbore_left: 0,
                counterbore_right: 0,
                length: 10000,
            }),
            config_cache: None,
        }));
        // log(&format!("{:#?}", design));
    }

    #[wasm_bindgen_test]
    fn config_instance_inv_operate_test() {
        let lib = ComponentLib::default();
        let mut design = DesignSpace::new();

        let add = AddInstance::extrude(lib.components.get("LCF8-4040").unwrap(), 100000).unwrap();
        design.push(DesignOperation::AddInstance(add));
        design.push(DesignOperation::PostProcessInstance(PostProcessInstance {
            id: design.instances[0].id,
            config: InstanceConfig::Extrude(ExtrudeConfig {
                drill_left: true,
                drill_right: false,
                bevel_cut: None,
                wrench_hole_left: None,
                wrench_hole_right: None,
                counterbore_left: 0,
                counterbore_right: 0,
                length: 10000,
            }),
            config_cache: None,
        }));
        design.pop();
        // log(&format!("{:#?}", design));
        design.repush();
        // log(&format!("{:#?}", design));
    }

    #[wasm_bindgen_test]
    fn profile_length_operate_test() {
        let lib = ComponentLib::default();
        let mut design = DesignSpace::new();

        let add = AddInstance::extrude(lib.components.get("LCF8-4040").unwrap(), 100000).unwrap();
        design.push(DesignOperation::AddInstance(add));
        design.push(DesignOperation::ExtrudeAddLength(ExtrudeAddLength {
            id: design.instances[0].id,
            dlength: 100000,
            matrix: Isometry3::translation(100.0, 100.0, 100.0),
        }));
        design.push(DesignOperation::ExtrudeAddLength(ExtrudeAddLength {
            id: design.instances[0].id,
            dlength: -100000,
            matrix: Isometry3::translation(100.0, 100.0, 100.0),
        }));
        // log(&format!("{:#?}", design));
    }

    #[wasm_bindgen_test]
    fn profile_length_inv_operate_test() {
        let lib = ComponentLib::default();
        let mut design = DesignSpace::new();

        let add = AddInstance::extrude(lib.components.get("LCF8-4040").unwrap(), 100000).unwrap();
        design.push(DesignOperation::AddInstance(add));
        design.push(DesignOperation::ExtrudeAddLength(ExtrudeAddLength {
            id: design.instances[0].id,
            dlength: 100000,
            matrix: Isometry3::translation(100.0, 100.0, 100.0),
        }));
        design.pop();
        log(&format!("{:#?}", design));
        design.repush();
        log(&format!("{:#?}", design));
    }

    #[wasm_bindgen_test]
    fn profile_length_compress_test() {
        let lib = ComponentLib::default();
        let mut design = DesignSpace::new();

        let add = AddInstance::extrude(lib.components.get("LCF8-4040").unwrap(), 100000).unwrap();
        design.push(DesignOperation::AddInstance(add));
        design.push(DesignOperation::ExtrudeAddLength(ExtrudeAddLength {
            id: design.instances[0].id,
            dlength: 100000,
            matrix: Isometry3::translation(100.0, 100.0, 100.0),
        }));
        design.push(DesignOperation::ExtrudeAddLength(ExtrudeAddLength {
            id: design.instances[0].id,
            dlength: 100000,
            matrix: Isometry3::translation(100.0, 100.0, 100.0),
        }));
        // log(&format!("{:#?}", design));
    }

    #[wasm_bindgen_test]
    fn move_instance_operate_test() {
        let lib = ComponentLib::default();
        let mut design = DesignSpace::new();

        let add = AddInstance::extrude(lib.components.get("LCF8-4040").unwrap(), 100000).unwrap();
        design.push(DesignOperation::AddInstance(add));
        design.push(DesignOperation::MoveInstance(MoveInstance {
            id: design.instances[0].id,
            matrix: Isometry3::translation(100.0, 100.0, 100.0),
        }));
        // log(&format!("{:#?}", design));
    }

    #[wasm_bindgen_test]
    fn move_instance_inv_operate_test() {
        let lib = ComponentLib::default();
        let mut design = DesignSpace::new();

        let add = AddInstance::extrude(lib.components.get("LCF8-4040").unwrap(), 100000).unwrap();
        design.push(DesignOperation::AddInstance(add));
        design.push(DesignOperation::MoveInstance(MoveInstance {
            id: design.instances[0].id,
            matrix: Isometry3::translation(100.0, 100.0, 100.0),
        }));
        design.pop();
        // log(&format!("{:#?}", design));
        design.repush();
        // log(&format!("{:#?}", design));
    }

    #[wasm_bindgen_test]
    fn move_instance_compress_test() {
        let lib = ComponentLib::default();
        let mut design = DesignSpace::new();

        let add = AddInstance::extrude(lib.components.get("LCF8-4040").unwrap(), 100000).unwrap();
        design.push(DesignOperation::AddInstance(add));
        design.push(DesignOperation::MoveInstance(MoveInstance {
            id: design.instances[0].id,
            matrix: Isometry3::translation(100.0, 100.0, 100.0),
        }));
        design.push(DesignOperation::MoveInstance(MoveInstance {
            id: design.instances[0].id,
            matrix: Isometry3::translation(100.0, 100.0, 100.0),
        }));
        // log(&format!("{:#?}", design));
    }

    #[wasm_bindgen_test]
    fn panel_size_operate_test() {
        let lib = ComponentLib::default();
        let mut design = DesignSpace::new();

        let add = AddInstance::panel(
            lib.components.get("WoodenPanel-test").unwrap(),
            200000,
            100000,
            2000,
        )
        .unwrap();
        design.push(DesignOperation::AddInstance(add));
        // design.push(DesignOperation::PanelAddSize(PanelAddSize {
        //     id: design.instances[0].id,
        //     dwidth: 100,
        //     dheight: 100,
        //     dthickness: 10,
        // }));
        // log(&format!("{:#?}", design));
    }

    #[wasm_bindgen_test]
    fn panel_size_inv_operate_test() {
        let lib = ComponentLib::default();
        let mut design = DesignSpace::new();

        let add = AddInstance::panel(
            lib.components.get("WoodenPanel-test").unwrap(),
            200000,
            100000,
            2000,
        )
        .unwrap();
        design.push(DesignOperation::AddInstance(add));
        // design.push(DesignOperation::PanelAddSize(PanelAddSize {
        //     id: design.instances[0].id,
        //     dwidth: 100,
        //     dheight: 100,
        //     dthickness: 10,
        // }));
        design.pop();
        // log(&format!("{:#?}", design));
        design.repush();
        // log(&format!("{:#?}", design));
    }

    #[wasm_bindgen_test]
    fn panel_size_compress_test() {
        let lib = ComponentLib::default();
        let mut design = DesignSpace::new();

        let add = AddInstance::panel(
            lib.components.get("WoodenPanel-test").unwrap(),
            200000,
            100000,
            2000,
        )
        .unwrap();
        design.push(DesignOperation::AddInstance(add));
        design.push(DesignOperation::PanelAddSize(PanelAddSize {
            id: design.instances[0].id,
            dwidth: 100,
            dheight: 100,
            dthickness: 10,
            matrix: Isometry3::translation(100.0, 100.0, 100.0),
        }));
        design.push(DesignOperation::PanelAddSize(PanelAddSize {
            id: design.instances[0].id,
            dwidth: 100,
            dheight: 100,
            dthickness: 10,
            matrix: Isometry3::translation(100.0, 100.0, 100.0),
        }));
        // log(&format!("{:#?}", design));
    }
}
