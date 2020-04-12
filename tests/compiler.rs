//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use cao_lang_wasm::{compile, AstNode, CompilationUnit};
use wasm_bindgen::JsValue;

#[wasm_bindgen_test]
fn can_compile_simple_program() {
    let start_node = serde_json::json! {{
        "instruction": { "Start": null },
        "child": 1
    }};
    let start_node = JsValue::from_serde(&start_node).unwrap();
    let start_node: AstNode = start_node.into_serde().unwrap();

    let scalar_node = serde_json::json! {{
        "instruction": { "ScalarInt": 69 }
    }};
    let scalar_node = JsValue::from_serde(&scalar_node).unwrap();
    let scalar_node: AstNode = scalar_node.into_serde().unwrap();

    let cu = CompilationUnit::new()
        .with_node(0, start_node)
        .with_node(1, scalar_node);

    compile(&cu).unwrap();
}

#[wasm_bindgen_test]
fn check_error_returns_null_for_valid_node() {
    let node = serde_json::json! {{
        "instruction": { "ScalarInt": 69 }
        , "child": 420
        , "foo": "bar"
    }};
    let node = JsValue::from_serde(&node).unwrap();

    let res = AstNode::check_error(&node);

    assert!(res.is_null(), "Valid nodes should return null");
}

#[wasm_bindgen_test]
fn check_error_returns_error_for_invalid_node() {
    let node = serde_json::json! {{
        "instruction": { "Asd": {"a": "bar"} }
        , "children": [420, 123]
    }};
    let node = JsValue::from_serde(&node).unwrap();

    let res = AstNode::check_error(&node);

    assert!(!res.is_null(), "Invalid nodes should not return null");
}
