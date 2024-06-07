import { TransformControls } from "three/examples/jsm/Addons.js";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";
import { Renderer } from "./setup_threejs";
import { Design } from "./design";
import { Object3D, Matrix4, Vector3, Mesh, Raycaster, Vector2, MeshLambertMaterial, Group, CylinderGeometry, ConeGeometry, Color, MeshBasicMaterial, Quaternion } from "three";
import { ExtrudeConfig, Instance, PanelConfig } from "framead";
import { ViewportGizmo } from "./viewportgizmo/ViewportGizmo";
import { Atom, atom } from "jotai";
import { LengthControls } from "./lengthgizmo/lengthgizmo";
export interface Controls {
    transform_control: TransformControls;
    orbit_control: OrbitControls;
}

export class DesignControls extends EventTarget implements Controls {
    transform_control: TransformControls;
    control_tip: Object3D;

    orbit_control: OrbitControls;

    renderer: Renderer;
    design: Design;

    raycaster: Raycaster = new Raycaster();
    mesh: Mesh | null = null;

    left_control_length: LengthControls;
    left_control_length_tip: Object3D;
    right_control_length: LengthControls;
    right_control_length_tip: Object3D;

    pos_x_control: LengthControls;
    pos_x_tip: Object3D;
    neg_x_control: LengthControls;
    neg_x_tip: Object3D;
    pos_y_control: LengthControls;
    pos_y_tip: Object3D;
    neg_y_control: LengthControls;
    neg_y_tip: Object3D;
    constructor(design: Design, renderer: Renderer) {
        super();
        this.design = design;
        this.renderer = renderer;
        // orbit control
        this.orbit_control = new OrbitControls(renderer.camera, renderer.canvas);
        // this.orbit_control.enableDamping = true;

        // gizmo
        this.init_gizmo();

        // raycaster
        this.init_raycaster();

        // transform control
        let [transform_control, control_tip] = this.init_transform_control();
        this.transform_control = transform_control;
        this.control_tip = control_tip;

        // extrude length control
        let [left_control_length, left_control_length_tip, right_control_length, right_control_length_tip] = this.init_extrude_length_control();
        this.left_control_length = left_control_length as LengthControls;
        this.left_control_length_tip = left_control_length_tip as Object3D;
        this.right_control_length = right_control_length as LengthControls;
        this.right_control_length_tip = right_control_length_tip as Object3D;

        // panel size control
        let [pos_x_control, pos_x_tip, neg_x_control, neg_x_tip, pos_y_control, pos_y_tip, neg_y_control, neg_y_tip] = this.init_panel_size_control();
        this.pos_x_control = pos_x_control as LengthControls;
        this.pos_x_tip = pos_x_tip as Object3D;
        this.neg_x_control = neg_x_control as LengthControls;
        this.neg_x_tip = neg_x_tip as Object3D;
        this.pos_y_control = pos_y_control as LengthControls;
        this.pos_y_tip = pos_y_tip as Object3D;
        this.neg_y_control = neg_y_control as LengthControls;
        this.neg_y_tip = neg_y_tip as Object3D;
    }

    bind(mesh: Mesh | string) {
        this.mesh = typeof mesh === "string" ? this.design.render_space.instance_meshs.get(mesh) as Mesh : mesh;

        // update transform control
        this.update_control_tip();

        // update extrude length control
        this.update_length_control_tip();

        // update panel size control
        this.update_panel_size_control_tip();

        this.show_control();
        this.dispatchEvent(new CustomEvent("bind", { detail: this }));
    }

    unbind() {
        this.mesh = null;
        this.hide_control();
        this.dispatchEvent(new CustomEvent("unbind", { detail: this }));
    }

    private hide_control() {
        this.transform_control.enabled = false;
        this.transform_control.visible = false;
        this.left_control_length.enabled = false;
        this.left_control_length.visible = false;
        this.right_control_length.enabled = false;
        this.right_control_length.visible = false;
        this.pos_x_control.enabled = false;
        this.pos_x_control.visible = false;
        this.neg_x_control.enabled = false;
        this.neg_x_control.visible = false;
        this.pos_y_control.enabled = false;
        this.pos_y_control.visible = false;
        this.neg_y_control.enabled = false;
        this.neg_y_control.visible = false;
    }

    private show_control() {
        const instance = this.mesh?.userData as Instance;
        if (instance.component_type() == "Extrude") {
            this.left_control_length.enabled = true;
            this.left_control_length.visible = true;
            this.right_control_length.enabled = true;
            this.right_control_length.visible = true;
        }
        if (instance.component_type() == "Panel") {
            this.pos_x_control.enabled = true;
            this.pos_x_control.visible = true;
            this.neg_x_control.enabled = true;
            this.neg_x_control.visible = true;
            this.pos_y_control.enabled = true;
            this.pos_y_control.visible = true;
            this.neg_y_control.enabled = true;
            this.neg_y_control.visible = true;
        }
        this.transform_control.enabled = true;
        this.transform_control.visible = true;
    }

    private pick_instance_mesh(event: MouseEvent) {
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

    private init_transform_control(): [TransformControls, Object3D] {
        // transform control
        const control_tip = new Object3D();
        this.renderer.scene.add(control_tip);
        const transform_control = new TransformControls(this.renderer.camera, this.renderer.canvas);
        transform_control.setSize(0.7);
        transform_control.attach(control_tip);
        transform_control.setSpace("local");
        transform_control.addEventListener('mouseDown', () => this.orbit_control.enabled = false);
        transform_control.addEventListener('mouseUp', () => {
            this.orbit_control.enabled = true;
        });
        transform_control.enabled = false;
        transform_control.visible = false;
        transform_control.addEventListener("objectChange", () => {
            if (this.mesh) {
                this.design.move_instance(this.mesh.userData as Instance, control_tip.matrix);
                this.update_length_control_tip();
                this.update_panel_size_control_tip();
            }
        });
        this.renderer.scene.add(transform_control);

        return [transform_control, control_tip];
    }

    private update_control_tip() {
        this.mesh?.matrix.decompose(this.control_tip.position, this.control_tip.quaternion, new Vector3());
    }

    private init_raycaster() {
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
                    this.unbind();
                }
            }
        });
    }

    private init_gizmo() {
        const viewportGizmo = new ViewportGizmo(this.renderer.camera, this.renderer.renderer, {
            container: document.body,
        });
        viewportGizmo.target = this.orbit_control.target;
        viewportGizmo.addEventListener("start", () => (this.orbit_control.enabled = false));
        viewportGizmo.addEventListener("end", () => (this.orbit_control.enabled = true));

        this.orbit_control.addEventListener("change", () => {
            viewportGizmo.update();
        });

        function animate() {
            viewportGizmo.render();
            requestAnimationFrame(animate);
        }
        animate();
    }

    private init_extrude_length_control() {
        const left_control_length_tip = new Object3D();
        this.renderer.scene.add(left_control_length_tip);
        const left_length_control = new LengthControls(this.renderer.camera, this.renderer.canvas);
        left_length_control.attach(left_control_length_tip);
        left_length_control.setSpace("local");
        left_length_control.addEventListener('mouseDown', () => this.orbit_control.enabled = false);
        left_length_control.addEventListener('mouseUp', () => {
            this.orbit_control.enabled = true;
        });
        left_length_control.addEventListener("objectChange", (e) => {
            if (this.mesh) {
                let dl = Math.floor(e.offset.x * 100000);
                this.design.extrude_add_length_dir(this.mesh.userData as Instance, dl, 1);
                this.update_control_tip();
            }
        });
        left_length_control.visible = false;
        left_length_control.enabled = false;
        this.renderer.scene.add(left_length_control);

        const right_control_length_tip = new Object3D();
        this.renderer.scene.add(right_control_length_tip);
        const right_length_control = new LengthControls(this.renderer.camera, this.renderer.canvas);
        right_length_control.attach(right_control_length_tip);
        right_length_control.setSpace("local");
        right_length_control.addEventListener('mouseDown', () => this.orbit_control.enabled = false);
        right_length_control.addEventListener('mouseUp', () => {
            this.orbit_control.enabled = true;
        });
        right_length_control.addEventListener("objectChange", (e) => {
            if (this.mesh) {
                let dl = Math.floor(e.offset.x * 100000);
                this.design.extrude_add_length_dir(this.mesh.userData as Instance, dl, -1);
                this.update_control_tip();
            }
        });
        right_length_control.visible = false;
        right_length_control.enabled = false;
        this.renderer.scene.add(right_length_control);
        return [left_length_control, left_control_length_tip, right_length_control, right_control_length_tip];
    }

    private update_length_control_tip() {
        const instance = this.mesh?.userData as Instance;
        if (instance.component_type() == "Extrude" && this.mesh) {
            this.mesh.matrix.decompose(this.left_control_length_tip.position, this.left_control_length_tip.quaternion, new Vector3());
            let length = (instance.instance_config() as { Extrude: ExtrudeConfig }).Extrude.length / 100000;
            this.left_control_length_tip.translateX(length / 2);

            this.mesh.matrix.decompose(this.right_control_length_tip.position, this.right_control_length_tip.quaternion, new Vector3())
            this.right_control_length_tip.rotateY(Math.PI);
            this.right_control_length_tip.translateX(length / 2);
        }
    }

    private init_panel_size_control() {
        // pos x
        const pos_x_tip = new Object3D();
        this.renderer.scene.add(pos_x_tip);
        const pos_x_control = new LengthControls(this.renderer.camera, this.renderer.canvas);
        pos_x_control.attach(pos_x_tip);
        pos_x_control.setSpace("local");
        pos_x_control.addEventListener('mouseDown', () => this.orbit_control.enabled = false);
        pos_x_control.addEventListener('mouseUp', () => {
            this.orbit_control.enabled = true;
        });
        pos_x_control.addEventListener("objectChange", (e) => {
            if (this.mesh) {
                let dx = Math.floor(e.offset.x * 100000);
                this.design.panel_add_size_dir(this.mesh.userData as Instance, dx, 0, 0, 1, 0);
                this.update_control_tip();
                this.update_panel_size_control_tip_y();
            }
        });
        pos_x_control.visible = false;
        pos_x_control.enabled = false;
        this.renderer.scene.add(pos_x_control);

        // neg x
        const neg_x_tip = new Object3D();
        this.renderer.scene.add(neg_x_tip);
        const neg_x_control = new LengthControls(this.renderer.camera, this.renderer.canvas);
        neg_x_control.attach(neg_x_tip);
        neg_x_control.setSpace("local");
        neg_x_control.addEventListener('mouseDown', () => this.orbit_control.enabled = false);
        neg_x_control.addEventListener('mouseUp', () => {
            this.orbit_control.enabled = true;
        });
        neg_x_control.addEventListener("objectChange", (e) => {
            if (this.mesh) {
                let dx = Math.floor(e.offset.x * 100000);
                this.design.panel_add_size_dir(this.mesh.userData as Instance, dx, 0, 0, -1, 0);
                this.update_control_tip();
                this.update_panel_size_control_tip_y();
            }
        });
        neg_x_control.visible = false;
        neg_x_control.enabled = false;
        this.renderer.scene.add(neg_x_control);

        // pos y
        const pos_y_tip = new Object3D();
        this.renderer.scene.add(pos_y_tip);
        const pos_y_control = new LengthControls(this.renderer.camera, this.renderer.canvas);
        pos_y_control.attach(pos_y_tip);
        pos_y_control.setSpace("local");
        pos_y_control.addEventListener('mouseDown', () => this.orbit_control.enabled = false);
        pos_y_control.addEventListener('mouseUp', () => {
            this.orbit_control.enabled = true;
        });
        pos_y_control.addEventListener("objectChange", (e) => {
            if (this.mesh) {
                let dy = Math.floor(e.offset.x * 100000);
                this.design.panel_add_size_dir(this.mesh.userData as Instance, 0, dy, 0, 0, -1);
                this.update_control_tip();
                this.update_panel_size_control_tip_x();
            }
        });
        pos_y_control.visible = false;
        pos_y_control.enabled = false;
        this.renderer.scene.add(pos_y_control);

        // neg y
        const neg_y_tip = new Object3D();
        this.renderer.scene.add(neg_y_tip);
        const neg_y_control = new LengthControls(this.renderer.camera, this.renderer.canvas);
        neg_y_control.attach(neg_y_tip);
        neg_y_control.setSpace("local");
        neg_y_control.addEventListener('mouseDown', () => this.orbit_control.enabled = false);
        neg_y_control.addEventListener('mouseUp', () => {
            this.orbit_control.enabled = true;
        });
        neg_y_control.addEventListener("objectChange", (e) => {
            if (this.mesh) {
                let dy = Math.floor(e.offset.x * 100000);
                this.design.panel_add_size_dir(this.mesh.userData as Instance, 0, dy, 0, 0, 1);
                this.update_control_tip();
                this.update_panel_size_control_tip_x();
            }
        });
        neg_y_control.visible = false;
        neg_y_control.enabled = false;
        this.renderer.scene.add(neg_y_control);

        return [pos_x_control, pos_x_tip, neg_x_control, neg_x_tip, pos_y_control, pos_y_tip, neg_y_control, neg_y_tip];
    }

    private update_panel_size_control_tip_x() {
        const instance = this.mesh?.userData as Instance;
        if (instance.component_type() == "Panel" && this.mesh) {
            let config = (instance.instance_config() as { Panel: PanelConfig }).Panel;
            this.mesh.matrix.decompose(this.pos_x_tip.position, this.pos_x_tip.quaternion, new Vector3());
            this.pos_x_tip.translateX(config.x / 100000 / 2);

            this.mesh.matrix.decompose(this.neg_x_tip.position, this.neg_x_tip.quaternion, new Vector3())
            this.neg_x_tip.rotateY(Math.PI);
            this.neg_x_tip.translateX(config.x / 100000 / 2);
        }
    }

    private update_panel_size_control_tip_y() {
        const instance = this.mesh?.userData as Instance;
        if (instance.component_type() == "Panel" && this.mesh) {
            let config = (instance.instance_config() as { Panel: PanelConfig }).Panel;
            this.mesh.matrix.decompose(this.pos_y_tip.position, this.pos_y_tip.quaternion, new Vector3());
            this.pos_y_tip.rotateY(Math.PI / 2);
            this.pos_y_tip.translateX(config.y / 100000 / 2);

            this.mesh.matrix.decompose(this.neg_y_tip.position, this.neg_y_tip.quaternion, new Vector3())
            this.neg_y_tip.rotateY(-Math.PI / 2);
            this.neg_y_tip.translateX(config.y / 100000 / 2);
        }
    }

    private update_panel_size_control_tip() {
        this.update_panel_size_control_tip_x();
        this.update_panel_size_control_tip_y();
    }
}