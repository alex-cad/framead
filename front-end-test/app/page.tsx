"use client"

import wasm_init from "framead";
import { MouseEvent, useEffect, useRef, useState } from "react";
import { setup_threejs } from "./lib/setup_threejs";
import { init_design, Design } from "./lib/design";
import { DesignControls } from "./lib/controls";

export default function Home() {
  const canvas_ref = useRef(null);
  const initialized = useRef(false);

  const design = useRef<Design | null>(null);
  const controls = useRef<DesignControls | null>(null);

  const [instances, setInstances] = useState<{ name: string, id: string }[]>([]);

  useEffect(() => {
    if (!initialized.current && canvas_ref.current) {
      initialized.current = true
      const renderer = setup_threejs(canvas_ref.current);
      wasm_init().then(init_design).then((init_design) => {
        design.current = init_design;
        renderer.scene.add(init_design.render_space.group);
        controls.current = new DesignControls(init_design, renderer);
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
    controls.current?.transform_control.setMode("rotate");
  }

  const handleTranslationControlMode = (e: MouseEvent) => {
    e.stopPropagation();
    controls.current?.transform_control.setMode("translate");
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
