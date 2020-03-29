//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use cao_lang_wasm::{compile, AstNode, CompilationUnit};
use wasm_bindgen::JsValue;

#[wasm_bindgen_test]
fn can_compile_simple_program() {
    let start_node: serde_json::Value = serde_json::json! {{
        "Start": null
    }};
    let start_node = JsValue::from_serde(&start_node).unwrap();
    let mut start_node = AstNode::new(start_node);
    start_node.child = Some(1);

    let scalar_node: serde_json::Value = serde_json::json! {{
        "ScalarInt": {
          "value": 69
        }
    }};
    let scalar_node = JsValue::from_serde(&scalar_node).unwrap();
    let scalar_node = AstNode::new(scalar_node);

    let mut cu: CompilationUnit = CompilationUnit::new();
    cu.set_node(0, &start_node);
    cu.set_node(1, &scalar_node);

    compile(&cu).unwrap();
}
