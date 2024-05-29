use nalgebra::Isometry3;
use uuid::Uuid;

use crate::{
    component::Component,
    instance::{Instance, InstanceConfig},
    DesignSpace,
};

/// an edit operation that can be applied to a target
pub trait Operation {
    type Target;
    fn operate(&mut self, target: &mut Self::Target);
    fn inverse(&mut self, target: &mut Self::Target);
    fn compress(&mut self, target: &Self) -> bool;
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ProfileLength {
    pub(crate) id: Uuid,
    pub(crate) dlength: i32,
}

impl Operation for ProfileLength {
    type Target = DesignSpace;

    fn operate(&mut self, target: &mut Self::Target) {
        let index = target.instances.iter().position(|i| i.id == self.id);
        if let Some(index) = index {
            if let InstanceConfig::Extrude(e) = &mut target.instances[index].config {
                e.length = ((e.length as i32) + self.dlength) as u32;
            }
        }
    }

    fn inverse(&mut self, target: &mut Self::Target) {
        let index = target.instances.iter().position(|i| i.id == self.id);
        if let Some(index) = index {
            if let InstanceConfig::Extrude(e) = &mut target.instances[index].config {
                e.length = ((e.length as i32) - self.dlength) as u32;
            }
        }
    }

    fn compress(&mut self, target: &Self) -> bool {
        self.dlength += target.dlength;
        true
    }
}

#[derive(Debug)]
pub struct PanelSize {
    pub(crate) id: Uuid,
    pub(crate) dwidth: i32,
    pub(crate) dheight: i32,
    pub(crate) dthickness: i32,
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

#[derive(Debug)]
pub struct MoveInstance {
    pub(crate) id: Uuid,
    pub(crate) matrix: Isometry3<f32>,
}

impl Operation for MoveInstance {
    type Target = DesignSpace;

    fn operate(&mut self, target: &mut Self::Target) {
        let index = target.instances.iter().position(|i| i.id == self.id);
        if let Some(index) = index {
            target.instances[index].matrix *= self.matrix.to_homogeneous();
        }
    }

    fn inverse(&mut self, target: &mut Self::Target) {
        let index = target.instances.iter().position(|i| i.id == self.id);
        if let Some(index) = index {
            target.instances[index].matrix *= self.matrix.inverse().to_homogeneous();
        }
    }

    fn compress(&mut self, target: &Self) -> bool {
        self.matrix *= target.matrix;
        true
    }
}

#[derive(Debug)]
pub enum DesignOperation {
    AddInstance(AddInstance),
    RemoveInstance(RemoveInstance),
    PostProcessInstance(PostProcessInstance),
    ProfileLength(ProfileLength),
    PanelSize(PanelSize),
    MoveInstance(MoveInstance),
    // AddConstraint,
    // RemoveConstraint,
    // ConfigConstraint,
    // AddInput,
    // RemoveInput,
    // ConfigInput,
}

impl Operation for DesignOperation {
    type Target = DesignSpace;

    fn operate(&mut self, target: &mut Self::Target) {
        match self {
            DesignOperation::AddInstance(op) => op.operate(target),
            DesignOperation::RemoveInstance(op) => op.operate(target),
            DesignOperation::PostProcessInstance(op) => op.operate(target),
            DesignOperation::ProfileLength(op) => op.operate(target),
            DesignOperation::PanelSize(op) => op.operate(target),
            DesignOperation::MoveInstance(op) => op.operate(target),
        }
    }

    fn inverse(&mut self, target: &mut Self::Target) {
        match self {
            DesignOperation::AddInstance(op) => op.inverse(target),
            DesignOperation::RemoveInstance(op) => op.inverse(target),
            DesignOperation::PostProcessInstance(op) => op.inverse(target),
            DesignOperation::ProfileLength(op) => op.inverse(target),
            DesignOperation::PanelSize(op) => op.inverse(target),
            DesignOperation::MoveInstance(op) => op.inverse(target),
        }
    }

    fn compress(&mut self, target: &Self) -> bool {
        match (self, target) {
            (DesignOperation::ProfileLength(op), DesignOperation::ProfileLength(target)) => {
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
