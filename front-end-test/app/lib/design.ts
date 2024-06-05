import wasm_init, { DesignSpace, Component, Vender, Instance, ExtrudeConfig, PanelConfig, add_extrude_instance, move_instance, remove_instance, add_normal_instance, add_panel_instance, extrude_post_process, extrude_add_length, panel_add_size } from "framead";
import { MetalMaterial } from "./materials";
import { Vector3, Group, Material, Mesh, Matrix4, Quaternion } from "three";
import { STLLoader } from "three/examples/jsm/Addons.js";

interface MyComponent {
    config_data: Component;
    mesh_url: string;
    material: Material;
}

export class ComponentLib {
    map: Map<string, MyComponent>;
    constructor() {
        this.map = new Map();
    }

    set_component(name: string, component: MyComponent) {
        this.map.set(name, component);
    }

    get_component(name: string) {
        return this.map.get(name);
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
            new STLLoader().load(value.mesh_url, (geometry) => {
                const mesh = new Mesh(geometry, value.material);
                this.component_mesh_lib.set(label, mesh);
            });
        }
    }

    move_instance(instance: Instance, m: Matrix4) {
        // design.get_instances().forEach((instance) => {
        // });
        // let mesh = this.instance_meshs.get(instance.id());
        // if (mesh) {
        //   const translation = instance.trans();
        //   mesh.position.set(translation.x, translation.y, translation.z);
        //   const quat = instance.quat();
        //   mesh.quaternion.set(quat.i, quat.j, quat.k, quat.w);
        // }
    }

    rebuild(design: DesignSpace) {
        design.get_instances().forEach((instance) => {
            let mesh = this.instance_meshs.get(instance.id());

            if (mesh) {
                // 可能改变了位置与配置
                mesh.name = "stay";
                if (!instance.is_equal(mesh.userData as Instance)) {
                    if (!instance.is_matrix_equal(mesh.userData as Instance)) {
                        const translation = instance.trans();
                        mesh.position.set(translation.x, translation.y, translation.z);
                        const quat = instance.quat();
                        mesh.quaternion.set(quat.i, quat.j, quat.k, quat.w);
                    }
                    if (!instance.is_config_equal(mesh.userData as Instance)) {
                        const type = instance.component_type();
                        if (type == "Extrude") {
                            const config = instance.instance_config();
                            let extrude_config = (config as { Extrude: ExtrudeConfig }).Extrude;

                            // extrude length
                            // 0.01mm 100mm
                            let x_scale = extrude_config.length / 100 / 100;
                            mesh.scale.set(x_scale, 1, 1);

                        } else if (type == "Panel") {
                            const config = instance.instance_config();
                            let panel_config = (config as { Panel: PanelConfig }).Panel;

                            // let panel = instance.component_panel();
                            // let panel_size = panel.size;
                            // this.group.add(m);
                        }
                    }
                }
            } else {
                // 找不到mesh，新加入的mesh
                const label = instance.label();
                const mesh = this.component_mesh_lib.get(label);
                if (mesh) {
                    let m = mesh.clone();
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
                        // 0.01mm 100mm
                        let x_scale = extrude_config.length / 100 / 100;
                        m.scale.set(x_scale, 1, 1);

                    } else if (type == "Panel") {
                        const config = instance.instance_config();
                        let panel_config = (config as { Panel: PanelConfig }).Panel;

                        // let panel = instance.component_panel();
                        // let panel_size = panel.size;
                        // this.group.add(m);
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

    add_panel(label: string, width: number, height: number, thickness: number) {
        let component = this.component_lib.get_component(label)?.config_data;
        if (!component) {
            return;
        }
        this.design_space.push(add_panel_instance(component, width, height, thickness));
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

    extrude_add_length(instance: Instance, dlength: number, m: Matrix4) {
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
    const metal_material = new MetalMaterial();
    await metal_material.init_env_map();
    await wasm_init();

    const component_lib = new ComponentLib();
    let misumi = new Vender("misumi");
    let lcf8 = new Component("LCF8-4040", "LCF8-4040", {
        Extrude: {
            standard: {
                series: { S40: "SlotDepth12_3mm" },
                metarial: "_6063T5",
                surface: "AA10",
            },
            shape: {
                name: "LCF8-4040",
                shape: { Square: "FourSlot" },
                holes_count: 1,
            },
            post_process: {
                drill: "M8_25mm",
                bevel_cut: true,
                wrench_hole: true,
                wrench_hole_size: 7,
                counterbore: true,
                counterbore_size: "Z8",
                length: {
                    min: 5000,
                    max: 400000,
                    step: 50,
                },
            },
        }
    }, misumi);
    component_lib.set_component("LCF8-4040", {
        config_data: lcf8,
        mesh_url: "./LCF8-4040-Meter-Low.stl",
        material: metal_material.material,
    });
    // let footer = new Component("C-FMJ60-N", "C-FMJ60-N", {
    //     Floor: { Wheel: { Fuma: "_60F" } }
    // }, misumi);
    // component_lib.set_component("C-FMJ60-N", {
    //     config_data: footer,
    //     mesh_url: "./C-FMJ60-N-Meter-Low.stl",
    //     material: metal_material.material,
    // });
    const render_space = new RenderSpace(component_lib);
    return new Design(new DesignSpace(), render_space, component_lib);
}