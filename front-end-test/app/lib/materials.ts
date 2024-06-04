import { MeshStandardMaterial, TextureLoader, EquirectangularReflectionMapping, SRGBColorSpace } from "three";

export class MetalMaterial {
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