"use client"

import wasm_init, { DesignSpace, Component, Vender, Instance, ExtrudeConfig, PanelConfig, AddInstance, add_extrude_instance, move_instance } from "framead";
import { atom } from "jotai";

import { MouseEvent, useCallback, useEffect, useRef, useState } from "react";
import { Scene, WebGLRenderer, PerspectiveCamera, Vector3, DirectionalLight, PCFSoftShadowMap, Color, EquirectangularReflectionMapping, Group, InstancedMesh, Material, MeshStandardMaterial, TextureLoader, SRGBColorSpace, GridHelper, AxesHelper, Mesh, Object3D, Raycaster, Vector2, Matrix4, Quaternion } from "three";
import { STLLoader, TransformControls } from "three/examples/jsm/Addons.js";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";
import { div, label } from "three/examples/jsm/nodes/Nodes.js";
import { useImmer } from "use-immer";

interface Renderer {
  camera: PerspectiveCamera;
  canvas: HTMLCanvasElement;
  scene: Scene;
}

function setup_fullscreen_threejs(canvas: HTMLCanvasElement): Renderer {
  // renderer
  const renderer = new WebGLRenderer({
    canvas: canvas,
    antialias: true,
    alpha: false,
  })
  renderer.setPixelRatio(window.devicePixelRatio);
  renderer.setSize(window.innerWidth, window.innerHeight);
  renderer.shadowMap.enabled = true;
  renderer.shadowMap.type = PCFSoftShadowMap;

  // scene
  const scene = new Scene();
  scene.background = new Color(0x24283b);
  // scene.fog = new Fog(0xa0a0a0, 10, 500);

  // camera
  const camera = new PerspectiveCamera(45, window.innerWidth / window.innerHeight, 0.001, 1000);
  camera.position.set(1, 1, 1);
  camera.lookAt(new Vector3(0, 0, 0));

  // control


  // light
  // const hemiLight = new HemisphereLight(0xffffff, 0x8d8d8d, 3);
  // hemiLight.position.set(0, 100, 0);
  // scene.add(hemiLight);

  const dirLight = new DirectionalLight(0xffffff, 3);
  dirLight.position.set(- 0, 40, 50);
  dirLight.castShadow = true;
  dirLight.shadow.camera.top = 50;
  dirLight.shadow.camera.bottom = - 25;
  dirLight.shadow.camera.left = - 25;
  dirLight.shadow.camera.right = 25;
  dirLight.shadow.camera.near = 0.1;
  dirLight.shadow.camera.far = 200;
  dirLight.shadow.mapSize.set(1024, 1024);
  scene.add(dirLight);

  // ground
  // const ground = new Mesh(new PlaneGeometry(1000, 1000), new MeshPhongMaterial({ color: 0xcbcbcb, depthWrite: false }));
  // ground.rotation.x = - Math.PI / 2;
  // ground.position.y = 0;
  // ground.receiveShadow = true;
  // scene.add(ground);

  // helper
  const axesHelper = new AxesHelper(1);
  scene.add(axesHelper);
  const gridHelper = new GridHelper(10, 10, 0x888888, 0x444444);
  scene.add(gridHelper);

  // render loop
  const render = function () {
    requestAnimationFrame(render);
    renderer.render(scene, camera);
  }
  const onWindowResize = function () {
    const width = window.innerWidth;
    const height = window.innerHeight;
    camera.aspect = width / height;
    camera.updateProjectionMatrix();
    renderer.setSize(width, height);
  }
  window.addEventListener('resize', onWindowResize);
  render()
  return { scene, camera, canvas };
}

class MetalMaterial {
  material: MeshStandardMaterial;
  constructor() {
    this.material = new MeshStandardMaterial({
      color: 0xe3e3e3,
      emissive: 0x404040,
      roughness: 0.2,
      metalness: 0.9,
    });
  }

  async init_env_map() {
    let env_texture = await new TextureLoader().loadAsync("./airport.jpg");
    env_texture.mapping = EquirectangularReflectionMapping;
    env_texture.colorSpace = SRGBColorSpace;
    this.material.envMap = env_texture;
    this.material.needsUpdate = true;
  }
}

interface MyComponent {
  config_data: Component;
  mesh_url: string;
  material: Material;
}

class ComponentLib {
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



class RenderSpace {
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

class Design {
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
    // this.rebuild_render_space();
  }

  rebuild_render_space() {
    this.render_space.rebuild(this.design_space);
  }
}

async function init_design(): Promise<Design> {
  const metal_material = new MetalMaterial();
  await metal_material.init_env_map();
  await wasm_init();
  const component_lib = new ComponentLib();
  let misumi = new Vender("misumi");
  let c = new Component("LCF8-4040", "LCF8-4040", {
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
    config_data: c,
    mesh_url: "./LCF8-4040-Meter-Low.stl",
    material: metal_material.material,
  });
  const render_space = new RenderSpace(component_lib);
  return new Design(new DesignSpace(), render_space, component_lib);
}

interface Controls {
  transform_control: TransformControls;
  orbit_control: OrbitControls;
}

function init_canvas_controls(design: Design, renderer: Renderer) {
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

export default function Home() {
  const canvas_ref = useRef(null);
  const initialized = useRef(false);

  const design = useRef<Design | null>(null);
  const transform_control = useRef<TransformControls | null>(null);

  const [instances, setInstances] = useState<{ name: string, id: string }[]>([]);

  useEffect(() => {
    if (!initialized.current && canvas_ref.current) {
      initialized.current = true
      const renderer = setup_fullscreen_threejs(canvas_ref.current);
      wasm_init().then(init_design).then((init_design) => {
        design.current = init_design;
        renderer.scene.add(init_design.render_space.group);
        const controls = init_canvas_controls(init_design, renderer);
        transform_control.current = controls.transform_control;
      });
    }
  }, []);

  const handleAddComponent = () => {
    design.current?.add_extrude("LCF8-4040", 100000);
    setInstances(design.current?.design_space.get_instances().map((instance) => {
      return {
        name: instance.label(),
        id: instance.id(),
      }
    }
    ) ?? []);
  }

  const handleRotationControlMode = (e: MouseEvent) => {
    e.stopPropagation();
    transform_control.current?.setMode("rotate");
  }

  const handleTranslationControlMode = (e: MouseEvent) => {
    e.stopPropagation();
    transform_control.current?.setMode("translate");
  }

  return (
    <div>
      <canvas ref={canvas_ref}></canvas>
      <div className="absolute top-0 left-0 z-10">
        <button className=" p-2 m-2 bg-slate-50 rounded" onClick={handleAddComponent} >添加零件</button>
        <button className=" p-2 m-2 bg-slate-50 rounded" onClick={handleRotationControlMode} >旋转控制模式</button>
        <button className=" p-2 m-2 bg-slate-50 rounded" onClick={handleTranslationControlMode} >移动控制模式</button>
        
        <div className="p-2 m-2 bg-slate-50">
          {
            instances.map((instance, index) => (
              <div key={index}>{instance.name} {instance.id}</div>
            ))
          }
        </div>
      </div>
    </div>
  );
}
