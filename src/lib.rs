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
    instruction: cao_lang::compiler::InstructionNode,
    pub child: Option<NodeId>,
}

#[wasm_bindgen(js_class=AstNode)]
impl AstNode {
    #[wasm_bindgen(constructor)]
    pub fn new(instruction: JsValue, child: Option<NodeId>) -> Result<AstNode, JsValue> {
        let instruction: cao_lang::compiler::InstructionNode =
            instruction.into_serde().map_err(|e| err_to_js(&e))?;
        let node = Self { instruction, child };
        Ok(node)
    }
}

impl AstNode {
    pub fn instruction(&self) -> &cao_lang::compiler::InstructionNode {
        &self.instruction
    }
}

#[wasm_bindgen(js_name=CompilationUnit, inspectable)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationUnit {
    inner: cao_lang::compiler::CompilationUnit,
}

#[wasm_bindgen(js_class=CompilationUnit)]
impl CompilationUnit {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: cao_lang::compiler::CompilationUnit::default(),
        }
    }

    #[wasm_bindgen(js_name=setNode)]
    pub fn set_node(&mut self, id: i32, node: &AstNode) {
        use cao_lang::compiler;
        let node = compiler::AstNode {
            node: node.instruction().clone(),
            child: node.child,
        };
        self.inner.nodes.insert(id, node);
    }
}

#[wasm_bindgen]
pub fn compile(compilation_unit: &CompilationUnit) -> Result<(), JsValue> {
    let cu = compilation_unit.inner.clone();
    cao_lang::compiler::Compiler::compile(cu)
        .map_err(|e| e.into())
        .map(|_| ())
}

fn err_to_js(e: &impl std::error::Error) -> JsValue {
    JsValue::from_serde(&format!("{:?}", e)).unwrap()
}
