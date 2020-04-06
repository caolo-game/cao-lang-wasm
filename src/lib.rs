use cao_lang::compiler as cc;
use cao_lang::compiler::NodeId;
use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

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

    #[wasm_bindgen(js_name=empty)]
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

#[wasm_bindgen(js_name=CompilationUnit, inspectable)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationUnit {
    #[wasm_bindgen(skip)]
    pub inner: cc::CompilationUnit,
}

#[wasm_bindgen(js_class=CompilationUnit)]
impl CompilationUnit {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: cc::CompilationUnit::default(),
        }
    }

    #[wasm_bindgen(js_name=nodeDel)]
    /// Remove the node by given ID and return it if it was in `this`
    pub fn del_node(&mut self, id: i32) -> Option<AstNode> {
        self.inner.nodes.remove(&id).map(|node| AstNode {
            child: node.child,
            instruction: node.node.clone(),
        })
    }

    #[wasm_bindgen(js_name=nodeGet)]
    pub fn get_node(&self, id: i32) -> Option<AstNode> {
        self.inner.nodes.get(&id).map(|node| AstNode {
            child: node.child,
            instruction: node.node.clone(),
        })
    }

    #[wasm_bindgen(js_name=nodeSet)]
    pub fn set_node(&mut self, id: i32, node: AstNode) {
        let child = node.child;
        let node = cc::AstNode {
            child,
            node: node.instruction,
        };
        self.inner.nodes.insert(id, node);
    }

    #[wasm_bindgen(js_name=withNode)]
    pub fn with_node(mut self, id: i32, node: AstNode) -> Self {
        self.set_node(id, node);
        self
    }
}

#[wasm_bindgen]
pub fn compile(compilation_unit: &CompilationUnit) -> Result<(), JsValue> {
    let cu = compilation_unit.inner.clone();
    cc::compile(cu)
        .map_err(|e| format!("{}", e).into())
        .map(|_| ())
}

fn err_to_js(e: impl std::error::Error) -> JsValue {
    JsValue::from_serde(&format!("{:?}", e)).unwrap()
}
