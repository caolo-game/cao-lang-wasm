pub mod ast_node;
pub mod compilation_unit;

use compilation_unit::CompilationUnit;

use cao_lang::compiler as cc;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
/// Init the error handling of the library
pub fn init_error_handling() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn compile(compilation_unit: &CompilationUnit) -> js_sys::Promise {
    let cu = compilation_unit.inner.clone();
    let f = async move {
        cc::compile(cu)
            .map_err(|e| format!("{}", e))
            .map_err(|e| JsValue::from_serde(&e).unwrap())
            .map(|_| JsValue::null())
    };
    wasm_bindgen_futures::future_to_promise(f)
}

fn err_to_js(e: impl std::error::Error) -> JsValue {
    JsValue::from_serde(&format!("{:?}", e)).unwrap()
}
