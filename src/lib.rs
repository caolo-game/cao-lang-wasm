mod utils;

use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
/// Init the error handling of the library
pub fn init_error_handling() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(js_name=AstNode, inspectable)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstNode {
    instruction: cao_lang::compiler::InstructionNode,
    pub child: Option<cao_lang::compiler::NodeId>,
}

#[wasm_bindgen(js_class=AstNode)]
impl AstNode {
    #[wasm_bindgen(constructor)]
    pub fn new(instruction: JsValue) -> Self {
        let instruction = instruction
            .into_serde()
            .expect("Expected InstructionNode instance");
        Self {
            instruction,
            child: None,
        }
    }
}
