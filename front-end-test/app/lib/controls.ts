import { TransformControls } from "three/examples/jsm/Addons.js";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";
import { Renderer } from "./setup_threejs";
import { Design } from "./design";
import { Object3D, Matrix4, Vector3, Mesh, Raycaster, Vector2 } from "three";
import { Instance } from "framead";
import { ViewportGizmo } from "./viewportgizmo/ViewportGizmo";
import { Atom, atom } from "jotai";

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
    last_matrix = new Matrix4();
    constructor(design: Design, renderer: Renderer) {
        super();
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
                    this.unbind();
                }
            }
        });

        this.transform_control.addEventListener("objectChange", () => {
            if (this.mesh) {
                this.control_tip.matrix.decompose(this.mesh.position, this.mesh.quaternion, new Vector3());
                this.design.move_instance(this.mesh.userData as Instance, this.last_matrix.invert().multiply(this.control_tip.matrix));
            }
            this.last_matrix.copy(this.control_tip.matrix);
        });

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

    bind(mesh: Mesh | string) {
        this.mesh = typeof mesh === "string" ? this.design.render_space.instance_meshs.get(mesh) as Mesh : mesh;
        this.mesh.matrix.decompose(this.control_tip.position, this.control_tip.quaternion, new Vector3());
        this.last_matrix.copy(this.control_tip.matrix);
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
    }

    private show_control() {
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
}