use wasm_bindgen::prelude::wasm_bindgen;

// mod adjustment;
mod assembly_node;
mod component;
mod design;
mod instance;
mod utils;

pub use component::Component;
pub use design::DesignSpace;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
