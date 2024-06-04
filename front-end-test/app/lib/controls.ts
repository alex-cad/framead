import { TransformControls } from "three/examples/jsm/Addons.js";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";
import { Renderer } from "./setup_threejs";
import { Design } from "./design";
import { Object3D, Matrix4, Vector3, Mesh, Raycaster, Vector2 } from "three";
import { Instance } from "framead";

export interface Controls {
    transform_control: TransformControls;
    orbit_control: OrbitControls;
}

export class DesignControls implements Controls {
    transform_control: TransformControls;
    control_tip: Object3D;
    orbit_control: OrbitControls;
    renderer: Renderer;
    design: Design;
    raycaster: Raycaster = new Raycaster();
    mesh: Mesh | null = null;
    last_matrix = new Matrix4();
    constructor(design: Design, renderer: Renderer) {
        this.design = design;
        this.renderer = renderer;
        // orbit control
        this.orbit_control = new OrbitControls(renderer.camera, renderer.canvas);

        // transform control
        this.control_tip = new Object3D();
        renderer.scene.add(this.control_tip);
        this.transform_control = new TransformControls(renderer.camera, renderer.canvas);
        this.transform_control.attach(this.control_tip);
        this.transform_control.setSpace("local");
        this.transform_control.addEventListener('mouseDown', () => this.orbit_control.enabled = false);
        this.transform_control.addEventListener('mouseUp', () => {
            this.orbit_control.enabled = true;
        });
        this.hide_control();
        renderer.scene.add(this.transform_control);

        // raycaster
        let pointer_move = 0;
        let is_mouse_down = false;
        this.renderer.canvas.addEventListener("mousedown", () => {
            pointer_move = 0;
            is_mouse_down = true;
        })
        this.renderer.canvas.addEventListener("mousemove", (e) => {
            if (is_mouse_down) {
                pointer_move += Math.abs(e.movementX) + Math.abs(e.movementY);
            }
        })
        this.renderer.canvas.addEventListener("mouseup", (event) => {
            is_mouse_down = false;
            if (pointer_move < 5) {
                const mesh = this.pick_instance_mesh(event);
                if (mesh) {
                    this.bind(mesh);
                } else {
                    this.hide_control();
                    this.unbind();
                }
            }
        });

        this.transform_control.addEventListener("objectChange", () => {
            if (this.mesh) {
                // this.control_tip.matrix.decompose(this.mesh.position, this.mesh.quaternion, new Vector3());
                this.design.move_instance(this.mesh.userData as Instance, this.last_matrix.invert().multiply(this.control_tip.matrix));
            }
            this.last_matrix.copy(this.control_tip.matrix);
        });
    }

    bind(mesh: Mesh) {
        this.mesh = mesh;
        this.mesh.matrix.decompose(this.control_tip.position, this.control_tip.quaternion, new Vector3());
        this.last_matrix.copy(this.control_tip.matrix);
        this.show_control();
    }

    unbind() {
        this.mesh = null;
        this.hide_control();
    }

    hide_control() {
        this.transform_control.enabled = false;
        this.transform_control.visible = false;
    }

    show_control() {
        this.transform_control.enabled = true;
        this.transform_control.visible = true;
    }

    pick_instance_mesh(event: MouseEvent) {
        const pointer = new Vector2();
        pointer.x = (event.clientX / window.innerWidth) * 2 - 1;
        pointer.y = - (event.clientY / window.innerHeight) * 2 + 1;
        this.raycaster.setFromCamera(pointer, this.renderer.camera);
        const intersects = this.raycaster.intersectObjects(this.design.render_space.group.children);
        if (intersects.length > 0) {
            const intersect = intersects[0];
            return intersect.object as Mesh;
        }
    }
}

export function init_canvas_controls(design: Design, renderer: Renderer) {
    // orbit control
    const orbit = new OrbitControls(renderer.camera, renderer.canvas);

    // transform control
    const control_tip = new Object3D();
    renderer.scene.add(control_tip);
    const control = new TransformControls(renderer.camera, renderer.canvas);
    control.attach(control_tip);
    control.setSpace("local");
    control.addEventListener('mouseDown', () => orbit.enabled = false);
    control.addEventListener('mouseUp', () => {
        orbit.enabled = true;
    });

    let last_matrix = new Matrix4();
    control.addEventListener("objectChange", () => {
        if (instance_mesh) {
            control_tip.matrix.decompose(instance_mesh.position, instance_mesh.quaternion, new Vector3());
        }

        handle_instance_move(last_matrix.invert().multiply(control_tip.matrix));
        last_matrix.copy(control_tip.matrix);
    });

    let instance: Instance | null = null;
    let instance_mesh: Mesh | null = null;
    let handle_instance_move = (m: Matrix4) => {
        if (instance) {
            design.move_instance(instance, m);
        }
    }
    const bind_instance = (i: Instance, mesh: Mesh) => {
        instance = i;
        instance_mesh = mesh;
    }
    const unbind_instance = () => {
        instance = null;
        instance_mesh = null;
    }

    const hide_control = () => {
        control.enabled = false;
        control.visible = false;
    }
    hide_control();
    renderer.scene.add(control);
    const set_control_tip = (m: Matrix4) => {
        last_matrix.copy(m);
        m.decompose(control_tip.position, control_tip.quaternion, new Vector3());
        control.enabled = true;
        control.visible = true;
    };

    // raycaster
    const raycaster = new Raycaster();
    const pointer = new Vector2();
    let pointer_move = 0;
    let is_mouse_down = false;
    window.addEventListener("mousedown", () => {
        pointer_move = 0;
        is_mouse_down = true;
    })
    window.addEventListener("mousemove", (e) => {
        if (is_mouse_down) {
            pointer_move += Math.abs(e.movementX) + Math.abs(e.movementY);
        }
    })
    renderer.canvas.addEventListener("mouseup", (event) => {
        is_mouse_down = false;
        if (pointer_move < 5) {
            pointer.x = (event.clientX / window.innerWidth) * 2 - 1;
            pointer.y = - (event.clientY / window.innerHeight) * 2 + 1;
            raycaster.setFromCamera(pointer, renderer.camera);
            const intersects = raycaster.intersectObjects(design.render_space.group.children);
            if (intersects.length > 0) {
                const intersect = intersects[0];
                set_control_tip(intersect.object.matrix);
                bind_instance(intersect.object.userData as Instance, intersect.object as Mesh);
            } else {
                hide_control();
                unbind_instance();
            }
        }
    });
    return { transform_control: control, orbit_control: orbit };
}