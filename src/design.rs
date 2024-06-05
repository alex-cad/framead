mod operation;
use std::collections::HashMap;

use uuid::Uuid;
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
    instances: HashMap<Uuid, Instance>,
    // constraints: Vec<ConstraintSystem>,
    records: Vec<DesignOperation>,

    poped: Vec<DesignOperation>,
}

#[wasm_bindgen]
impl DesignSpace {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        DesignSpace {
            instances: HashMap::new(),
            records: Vec::new(),
            poped: Vec::new(),
        }
    }

    pub fn get_instances(&self) -> Vec<Instance> {
        self.instances.values().cloned().collect()
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
        instance::{ExtrudeConfig, InstanceConfig},
        log,
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
        let id = add.instance.id;
        design.push(DesignOperation::AddInstance(add));

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
        let id = add.instance.id;
        design.push(DesignOperation::AddInstance(add));

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
        let id = add.instance.id;
        design.push(DesignOperation::AddInstance(add));
        design.push(DesignOperation::PostProcessInstance(PostProcessInstance {
            id,
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
        let id = add.instance.id;
        design.push(DesignOperation::AddInstance(add));
        design.push(DesignOperation::PostProcessInstance(PostProcessInstance {
            id,
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
        let id = add.instance.id;
        design.push(DesignOperation::AddInstance(add));
        design.push(DesignOperation::ExtrudeAddLength(ExtrudeAddLength {
            id,
            dlength: 100000,
            new_matrix: Isometry3::translation(100.0, 100.0, 100.0),
            old_matrix: None,
        }));
        design.push(DesignOperation::ExtrudeAddLength(ExtrudeAddLength {
            id,
            dlength: -100000,
            new_matrix: Isometry3::translation(100.0, 100.0, 200.0),
            old_matrix: None,
        }));
        // log(&format!("{:#?}", design));
    }

    #[wasm_bindgen_test]
    fn profile_length_inv_operate_test() {
        let lib = ComponentLib::default();
        let mut design = DesignSpace::new();

        let add = AddInstance::extrude(lib.components.get("LCF8-4040").unwrap(), 100000).unwrap();
        let id = add.instance.id;
        design.push(DesignOperation::AddInstance(add));
        design.push(DesignOperation::ExtrudeAddLength(ExtrudeAddLength {
            id,
            dlength: 100000,
            new_matrix: Isometry3::translation(100.0, 100.0, 100.0),
            old_matrix: None,
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
        let id = add.instance.id;
        design.push(DesignOperation::AddInstance(add));
        design.push(DesignOperation::ExtrudeAddLength(ExtrudeAddLength {
            id,
            dlength: 100000,
            new_matrix: Isometry3::translation(100.0, 100.0, 100.0),
            old_matrix: None,
        }));
        design.push(DesignOperation::ExtrudeAddLength(ExtrudeAddLength {
            id,
            dlength: 100000,
            new_matrix: Isometry3::translation(100.0, 100.0, 100.0),
            old_matrix: None,
        }));
        // log(&format!("{:#?}", design));
    }

    #[wasm_bindgen_test]
    fn move_instance_operate_test() {
        let lib = ComponentLib::default();
        let mut design = DesignSpace::new();

        let add = AddInstance::extrude(lib.components.get("LCF8-4040").unwrap(), 100000).unwrap();
        let id = add.instance.id;
        design.push(DesignOperation::AddInstance(add));
        design.push(DesignOperation::MoveInstance(MoveInstance {
            id,
            new_matrix: Isometry3::translation(100.0, 100.0, 100.0),
            old_matrix: None,
        }));
        // log(&format!("{:#?}", design));
    }

    #[wasm_bindgen_test]
    fn move_instance_inv_operate_test() {
        let lib = ComponentLib::default();
        let mut design = DesignSpace::new();

        let add = AddInstance::extrude(lib.components.get("LCF8-4040").unwrap(), 100000).unwrap();
        let id = add.instance.id;
        design.push(DesignOperation::AddInstance(add));
        design.push(DesignOperation::MoveInstance(MoveInstance {
            id,
            new_matrix: Isometry3::translation(100.0, 100.0, 100.0),
            old_matrix: None,
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
        let id = add.instance.id;
        design.push(DesignOperation::AddInstance(add));
        design.push(DesignOperation::MoveInstance(MoveInstance {
            id,
            new_matrix: Isometry3::translation(100.0, 100.0, 100.0),
            old_matrix: None,
        }));
        design.push(DesignOperation::MoveInstance(MoveInstance {
            id,
            new_matrix: Isometry3::translation(100.0, 100.0, 100.0),
            old_matrix: None,
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
        let id = add.instance.id;
        design.push(DesignOperation::AddInstance(add));
        design.push(DesignOperation::PanelAddSize(PanelAddSize {
            id,
            dwidth: 100,
            dheight: 100,
            dthickness: 10,
            new_matrix: Isometry3::translation(100.0, 100.0, 100.0),
            old_matrix: None,
        }));
        design.push(DesignOperation::PanelAddSize(PanelAddSize {
            id,
            dwidth: 100,
            dheight: 100,
            dthickness: 10,
            new_matrix: Isometry3::translation(100.0, 100.0, 100.0),
            old_matrix: None,
        }));
        // log(&format!("{:#?}", design));
    }
}
