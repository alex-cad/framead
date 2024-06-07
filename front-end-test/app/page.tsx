"use client"

import wasm_init, { Instance } from "framead";
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
  const [selectedInstance, setSelectedInstance] = useState<Instance | null>(null);

  useEffect(() => {
    if (!initialized.current && canvas_ref.current) {
      initialized.current = true
      const renderer = setup_threejs(canvas_ref.current);
      wasm_init().then(init_design).then((init_design) => {
        design.current = init_design;
        renderer.scene.add(init_design.render_space.group);
        controls.current = new DesignControls(init_design, renderer);
        controls.current.addEventListener("bind", (e) => {
          setSelectedInstance(((e.currentTarget as DesignControls).mesh?.userData as Instance))
        })
        controls.current.addEventListener("unbind", (e) => {
          setSelectedInstance(null)
        })
      });
    }
  }, []);

  const handleAddExtrude = () => {
    design.current?.add_extrude("LCF8-4040", 100000);
    setInstances(design.current?.design_space.get_instances().map((instance) => {
      return {
        name: instance.label(),
        id: instance.id(),
      }
    }) ?? []);
  }

  const handleAddFooter = () => {
    design.current?.add_normal_instance("C-FMJ60-N");
    setInstances(design.current?.design_space.get_instances().map((instance) => {
      return {
        name: instance.label(),
        id: instance.id(),
      }
    }) ?? []);
  }

  const handleAddPanel = () => {
    design.current?.add_panel("Dummy-Plywood-1", 200000, 100000, 2000);
    setInstances(design.current?.design_space.get_instances().map((instance) => {
      return {
        name: instance.label(),
        id: instance.id(),
      }
    }) ?? []);
  }

  const handleRemoveComponent = () => {
    setSelectedInstance(null);
    controls.current?.unbind();
    design.current?.remove_instance(selectedInstance!);
    setInstances(design.current?.design_space.get_instances().map((instance) => {
      return {
        name: instance.label(),
        id: instance.id(),
      }
    }) ?? []);
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
        <button className=" p-2 m-2 bg-slate-50 rounded" onClick={handleAddExtrude} >添加铝型材</button>
        <button className=" p-2 m-2 bg-slate-50 rounded" onClick={handleAddFooter} >添加地脚</button>
        <button className=" p-2 m-2 bg-slate-50 rounded" onClick={handleAddPanel} >添加面板</button>
        <button className=" p-2 m-2 bg-slate-50 rounded" onClick={handleTranslationControlMode} >移动控制模式</button>
        <button className=" p-2 m-2 bg-slate-50 rounded" onClick={handleRotationControlMode} >旋转控制模式</button>

        <button className=" p-2 m-2 bg-slate-50 rounded" onClick={()=>{}}>
          加载设计模版
        </button>
        <div className={(instances.length === 0 ? "hidden" : "") + "p-2 m-2 bg-slate-50"}>
          {
            instances.map((instance, index) => (
              <div key={index}>
                <button onClick={() => {
                controls.current?.bind(instance.id);
              }}>{instance.name} {instance.id}</button>
              </div>
            ))
          }
        </div>
        <div className={(selectedInstance === null ? `hidden` : ``) + ""}>
          <button className=" p-2 m-2 bg-slate-50 rounded" onClick={handleRemoveComponent}>删除{selectedInstance?.id()}</button>
        </div>
      </div>
    </div>
  );
}
