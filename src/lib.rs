use wasm_bindgen::prelude::wasm_bindgen;

// mod adjustment;
mod assembly_node;
mod component;
mod design;
mod instance;
mod utils;

// pub use component::{Component, ExtrudeData, Vender};
// pub use design::DesignSpace;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[allow(non_snake_case, clippy::empty_docs)]
mod trans_rot_types {
    use serde::{Deserialize, Serialize};
    use tsify::Tsify;

    #[derive(Debug, Tsify, Serialize, Deserialize)]
    #[tsify(into_wasm_abi, from_wasm_abi)]
    pub struct Translation {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    #[derive(Debug, Tsify, Serialize, Deserialize)]
    #[tsify(into_wasm_abi, from_wasm_abi)]
    pub struct Quaternion {
        pub i: f32,
        pub j: f32,
        pub k: f32,
        pub w: f32,
    }
}

pub use trans_rot_types::*;