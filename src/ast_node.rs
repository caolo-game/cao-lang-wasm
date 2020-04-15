use super::err_to_js;
use cao_lang::compiler as cc;
use cao_lang::compiler::NodeId;
use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=AstNode, inspectable)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstNode {
    #[wasm_bindgen(skip)]
    pub instruction: cc::InstructionNode,
    pub child: Option<NodeId>,
}

#[wasm_bindgen(js_class=AstNode)]
impl AstNode {
    #[wasm_bindgen(constructor)]
    pub fn new(instruction: JsValue, child: Option<NodeId>) -> Result<AstNode, JsValue> {
        let instruction: cc::InstructionNode = instruction.into_serde().map_err(err_to_js)?;
        let node = Self { instruction, child };
        Ok(node)
    }

    /// Check if the passed-in object is a valid AstNode.
    /// Returns null if it is, and an error otherwise.
    #[wasm_bindgen(js_name=checkError)]
    pub fn check_error(value: &JsValue) -> Option<String> {
        let parsed: Result<AstNode, _> = value.into_serde();
        match parsed {
            Ok(_) => None,
            Err(e) => Some(format!("{:?}", e)),
        }
    }

    #[wasm_bindgen]
    pub fn empty() -> Self {
        Self {
            instruction: cc::InstructionNode::Pass,
            child: None,
        }
    }

    #[wasm_bindgen(js_name=loadInstruction)]
    pub fn load_instructon(&self) -> JsValue {
        JsValue::from_serde(&self.instruction).unwrap()
    }
}
