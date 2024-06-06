import { DesignSpace, Component, Vender, Instance, ExtrudeConfig, PanelConfig, add_extrude_instance, move_instance, remove_instance, add_normal_instance, add_panel_instance, extrude_post_process, extrude_add_length, panel_add_size, misumi_4040_extrude } from "framead";
import { MaterialLib } from "./materials";
import { Vector3, Group, Material, Mesh, Matrix4, Quaternion, BufferGeometry, BoxGeometry } from "three";
import { STLLoader } from "three/examples/jsm/Addons.js";

interface MyComponent {
    config_data: Component;
    mesh: string | BufferGeometry;
    material: Material;
}

export class ComponentLib {
    map: Map<string, MyComponent>;
    constructor() {
        this.map = new Map();
    }

    set_component(label: string, component: MyComponent) {
        this.map.set(label, component);
    }

    get_component(label: string) {
        return this.map.get(label);
    }
}

export class RenderSpace {
    group: Group;
    component_mesh_lib: Map<string, Mesh>;
    instance_meshs: Map<string, Mesh>;
    constructor(component_lib: ComponentLib) {
        this.group = new Group();
        this.group.name = "render space";
        this.component_mesh_lib = new Map();
        this.instance_meshs = new Map();
        for (let [label, value] of component_lib.map) {
            if (value.mesh instanceof BufferGeometry) {
                const mesh = new Mesh(value.mesh, value.material);
                this.component_mesh_lib.set(label, mesh);
            } else {
                new STLLoader().load(value.mesh, (geometry) => {
                    const mesh = new Mesh(geometry, value.material);
                    this.component_mesh_lib.set(label, mesh);
                });
            }
        }
    }

    rebuild_all(design: DesignSpace) {
        this.group.clear();
        design.get_instances().forEach((instance) => {
            console.log("rebuild", instance)
            let m = this.component_mesh_lib.get(instance.label());
            if (m) {
                let mesh = m.clone();
                mesh.userData = instance;
                const type = instance.component_type();
                if (type == "Extrude") {
                    const config = instance.instance_config();
                    let extrude_config = (config as { Extrude: ExtrudeConfig }).Extrude;
                    // why? 单位：0.01mm stl模型长度：100mm
                    let x_scale = extrude_config.length / 10000;
                    mesh.scale.set(x_scale, 1, 1);

                } else if (type == "Panel") {
                    const config = instance.instance_config();
                    let panel_config = (config as { Panel: PanelConfig }).Panel;
                    mesh.geometry = new BoxGeometry(panel_config.x / 100000, panel_config.thickness / 100000, panel_config.y / 100000);
                }
                const translation = instance.trans();
                mesh.position.set(translation.x, translation.y, translation.z);
                const quat = instance.quat();
                mesh.quaternion.set(quat.i, quat.j, quat.k, quat.w);
                this.instance_meshs.set(instance.id(), mesh);
                this.group.add(mesh);
            }
        })
    }

    rebuild(design: DesignSpace) {
        design.get_instances().forEach((instance) => {
            let mesh = this.instance_meshs.get(instance.id());

            if (mesh) {
                // 可能改变了位置与配置
                mesh.name = "stay";

                function is_matrix_equal(instance: Instance, mesh: Instance) {
                    let quat = instance.quat();
                    let trans = instance.trans();
                    let mesh_quat = mesh.quat();
                    let mesh_trans = mesh.trans();
                    // return quat.i == mesh_quat.i && quat.j == mesh_quat.j && quat.k == mesh_quat.k && quat.w == mesh_quat.w &&
                    //     trans.x == mesh_trans.x && trans.y == mesh_trans.y && trans.z == mesh_trans.z;
                    return false;
                }

                function is_config_equal(instance: Instance, mesh: Instance) {
                    let config = instance.instance_config();
                    let mesh_config = mesh.instance_config();
                    // console.log(config);
                    // return config == mesh_config;
                    return false;
                }

                // 位置改变

                const translation = instance.trans();
                mesh.position.set(translation.x, translation.y, translation.z);
                const quat = instance.quat();
                mesh.quaternion.set(quat.i, quat.j, quat.k, quat.w);

                const type = instance.component_type();
                if (type == "Extrude") {
                    const config = instance.instance_config();
                    let extrude_config = (config as { Extrude: ExtrudeConfig }).Extrude;
                    // why? 单位：0.01mm stl模型长度：100mm
                    let x_scale = extrude_config.length / 10000;
                    mesh.scale.set(x_scale, 1, 1);

                } else if (type == "Panel") {
                    const config = instance.instance_config();
                    let panel_config = (config as { Panel: PanelConfig }).Panel;
                    mesh.geometry = new BoxGeometry(panel_config.x / 100000, panel_config.thickness / 100000, panel_config.y / 100000);
                }
                mesh.userData = instance;
            } else {
                // 找不到mesh，新加入的mesh
                const label = instance.label();
                const component_mesh = this.component_mesh_lib.get(label);
                if (component_mesh) {
                    let m = component_mesh.clone();
                    m.name = "stay";
                    m.userData = instance;
                    const translation = instance.trans();
                    m.position.set(translation.x, translation.y, translation.z);
                    const quat = instance.quat();
                    m.quaternion.set(quat.i, quat.j, quat.k, quat.w);
                    const type = instance.component_type();
                    if (type == "Extrude") {
                        const config = instance.instance_config();
                        let extrude_config = (config as { Extrude: ExtrudeConfig }).Extrude;

                        // extrude length
                        // why? 单位：0.01mm stl模型长度：100mm
                        let x_scale = extrude_config.length / 10000;
                        m.scale.set(x_scale, 1, 1);

                    } else if (type == "Panel") {
                        const config = instance.instance_config();
                        let panel_config = (config as { Panel: PanelConfig }).Panel;
                        m.geometry = new BoxGeometry(panel_config.x / 100000, panel_config.thickness / 100000, panel_config.y / 100000);
                    }
                    this.group.add(m);
                    this.instance_meshs.set(instance.id(), m);
                } else {
                    console.log("what? should never happend")
                }
            }
        });

        // 寻找没有stay标记的mesh，删除
        this.group.children.forEach((child) => {
            if (child.name != "stay") {
                this.group.remove(child);
                this.instance_meshs.delete((child.userData as Instance).id());
            } else {
                child.name = "";
            }
        });
    }
}

export class Design {
    design_space: DesignSpace;
    render_space: RenderSpace;
    component_lib: ComponentLib;
    constructor(design_space: DesignSpace, render_space: RenderSpace, component_lib: ComponentLib) {
        this.design_space = design_space;
        this.render_space = render_space;
        this.component_lib = component_lib;
    }

    add_extrude(label: string, length: number) {
        let component = this.component_lib.get_component(label)?.config_data;
        if (!component) {
            return;
        }
        this.design_space.push(add_extrude_instance(component, length));
        this.rebuild_render_space();
    }

    add_normal_instance(label: string) {
        let component = this.component_lib.get_component(label)?.config_data;
        if (!component) {
            return;
        }
        this.design_space.push(add_normal_instance(component));
        this.rebuild_render_space();
    }

    add_panel(label: string, x: number, z: number, thickness: number) {
        let component = this.component_lib.get_component(label)?.config_data;
        if (!component) {
            return;
        }
        this.design_space.push(add_panel_instance(component, x, z, thickness));
        this.rebuild_render_space();
    }

    remove_instance(instance: Instance) {
        this.design_space.push(remove_instance(instance));
        this.rebuild_render_space();
    }

    extrude_post_process(instance: Instance, config: ExtrudeConfig) {
        let component = this.component_lib.get_component(instance.label())?.config_data;
        if (!component) {
            return;
        }
        this.design_space.push(extrude_post_process(instance, component, config));
        this.rebuild_render_space();
    }

    extrude_add_length(instance: Instance, dlength: number, dir: number) {
        let trans = instance.trans();
        let quat = instance.quat();
        let m = new Matrix4().setPosition(new Vector3(trans.x, trans.y, trans.z));
        let rm = new Matrix4().makeRotationFromQuaternion(new Quaternion(quat.i, quat.j, quat.k, quat.w));
        m.multiply(rm);
        m.multiply(new Matrix4().setPosition(new Vector3(dir * dlength / 100000 / 2, 0, 0)));
        let translation = new Vector3();
        let quaternion = new Quaternion();
        m.decompose(translation, quaternion, new Vector3());
        
        this.design_space.push(extrude_add_length(
            instance, dlength,
            {
                ...translation,
            },
            {
                i: quaternion.x,
                j: quaternion.y,
                k: quaternion.z,
                w: quaternion.w,
            }
        )
        );
        this.rebuild_render_space();
    }

    extrude_add_right(instance: Instance, dlength: number) {
        this.extrude_add_length(instance, dlength, -1);
    }
    

    panel_add_size(instance: Instance, dwidth: number, dheight: number, dthickness: number, m: Matrix4) {
        let translation = new Vector3();
        let quaternion = new Quaternion();
        m.decompose(translation, quaternion, new Vector3());
        this.design_space.push(panel_add_size(
            instance, dwidth, dheight, dthickness,
            {
                ...translation,
            },
            {
                i: quaternion.x,
                j: quaternion.y,
                k: quaternion.z,
                w: quaternion.w,
            }
        )
        );
        this.rebuild_render_space();
    }

    move_instance(instance: Instance, m: Matrix4) {
        let translation = new Vector3();
        let quaternion = new Quaternion();
        m.decompose(translation, quaternion, new Vector3());
        this.design_space.push(move_instance(
            instance,
            {
                ...translation,
            },
            {
                i: quaternion.x,
                j: quaternion.y,
                k: quaternion.z,
                w: quaternion.w,
            }
        ));
        this.rebuild_render_space();
    }

    rebuild_render_space() {
        this.render_space.rebuild(this.design_space);
    }
}

export async function init_design(): Promise<Design> {
    const material_lib = new MaterialLib();
    await material_lib.init();
    const component_lib = new ComponentLib();
    component_lib.set_component("LCF8-4040", {
        config_data: misumi_4040_extrude("LCF8-4040", "LCF8-4040", new Vender("misumi")),
        mesh: "./LCF8-4040-Meter-Low.stl",
        material: material_lib.material_map.get("metal")!,
    });

    let footer = new Component("C-FMJ60-N", "C-FMJ60-N", {
        Floor: { Wheel: { Fuma: "_60F" } }
    }, new Vender("misumi"));
    component_lib.set_component("C-FMJ60-N", {
        config_data: footer,
        mesh: "./C-FMJ60-N-Meter-Low.stl",
        material: material_lib.material_map.get("wheel_footer")!,
    });

    let panel = new Component("Dummy-Plywood-1", "Dummy-Plywood-1", {
        Panel: "Wood"
    }, new Vender("Dummy Plywood Vendor"));
    component_lib.set_component("Dummy-Plywood-1", {
        config_data: panel,
        mesh: new BoxGeometry(1, 1, 1),
        material: material_lib.material_map.get("plywood")!,
    });
    const render_space = new RenderSpace(component_lib);
    return new Design(new DesignSpace(), render_space, component_lib);
}