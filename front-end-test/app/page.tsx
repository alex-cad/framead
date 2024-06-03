"use client"

import wasm_init, { DesignSpace, Component, Vender, Instance } from "framead";

import { useEffect, useRef } from "react";
import { Scene, WebGLRenderer, PerspectiveCamera, Vector3, DirectionalLight, PCFSoftShadowMap, Color, EquirectangularReflectionMapping, Group, InstancedMesh, Material, MeshStandardMaterial, TextureLoader, SRGBColorSpace, GridHelper, AxesHelper, Mesh } from "three";
import { STLLoader } from "three/examples/jsm/Addons.js";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";

function setup_fullscreen_threejs(canvas: HTMLCanvasElement): Scene {
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
  new OrbitControls(camera, canvas);

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
  // const axesHelper = new AxesHelper(1);
  // scene.add(axesHelper);
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
  return scene;
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
  mesh_map: Map<string, Mesh>
  constructor(component_lib: ComponentLib) {
    this.group = new Group();
    this.mesh_map = new Map();
    for (let [label, value] of component_lib.map) {
      new STLLoader().load(value.mesh_url, (geometry) => {
        const mesh = new Mesh(geometry, value.material);
        this.mesh_map.set(label, mesh);
      });
    }
  }

  update_mesh(design: DesignSpace, component_lib: ComponentLib) {
    this.group.clear();
    design.get_instances().forEach((instance) => {
      const label = instance.label();
      const mesh = this.mesh_map.get(label);
      if (mesh) {
        let m = mesh.clone();
        // mesh matrix

        // extrude length

        // panel size
        this.group.add();
      } else {
        console.log("what? should never happend")
      }
    });

    // for (const [label, instance] of instance_map) {
    //   let mesh = this.instanced_mesh_map.get(label);
    //   if (mesh) {
    //     mesh = new InstancedMesh(mesh.geometry, mesh.material, )
    //   } else {
    //     console.log("what? should never happend")
    //   }
    // }
  }
}

async function run(canvas: HTMLCanvasElement) {
  const scene = setup_fullscreen_threejs(canvas);
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
  scene.add(render_space.group);
  const design_space = new DesignSpace();
  console.log(scene);
}

export default function Home() {
  const canvas_ref = useRef(null);
  const initialized = useRef(false);
  useEffect(() => {
    if (!initialized.current && canvas_ref.current) {
      initialized.current = true
      run(canvas_ref.current).catch(console.error);
    }
  }, []);
  return (
    <div>
      <canvas ref={canvas_ref}></canvas>
    </div>
  );
}
