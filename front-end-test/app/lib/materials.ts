import { MeshStandardMaterial, TextureLoader, EquirectangularReflectionMapping, SRGBColorSpace } from "three";

export async function load_env_texture() {
  let env_texture = await new TextureLoader().loadAsync("./airport.jpg");
  env_texture.mapping = EquirectangularReflectionMapping;
  env_texture.colorSpace = SRGBColorSpace;
  return env_texture;
}

export class MaterialLib {
  material_map: Map<string, MeshStandardMaterial>;
  constructor() {
    this.material_map = new Map();
  }
  async init() {
    let env_texture = await load_env_texture();
    let metal = new MeshStandardMaterial({
      color: 0xe3e3e3,
      emissive: 0x404040,
      roughness: 0.2,
      metalness: 0.9,
    });
    metal.envMap = env_texture;
    metal.needsUpdate = true;
    this.material_map.set("metal", metal);

    let wheel_footer = new MeshStandardMaterial({
      color: 0x616161,
    });
    wheel_footer.envMap = env_texture;
    wheel_footer.needsUpdate = true;
    this.material_map.set("wheel_footer", wheel_footer);

    let plywood = new MeshStandardMaterial({
      color: 0xd3a173,
    });
    plywood.envMap = env_texture;
    plywood.needsUpdate = true;
    this.material_map.set("plywood", plywood);
  }
}