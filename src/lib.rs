use wasm_bindgen::prelude::wasm_bindgen;

// mod adjustment;
mod assembly_node;
mod component;
mod design;
mod instance;
mod utils;


// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

    impl Translation {
        pub fn identity() -> Self {
            Self {
                x: 0.,
                y: 0.,
                z: 0.,
            }
        }
    }

    #[derive(Debug, Tsify, Serialize, Deserialize)]
    #[tsify(into_wasm_abi, from_wasm_abi)]
    pub struct Quaternion {
        pub i: f32,
        pub j: f32,
        pub k: f32,
        pub w: f32,
    }

    impl Quaternion {
        pub fn identity() -> Self {
            Self {
                i: 0.,
                j: 0.,
                k: 0.,
                w: 1.,
            }
        }
    }
}

pub use trans_rot_types::*;
