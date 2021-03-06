// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.
use crate::ops::JsonResult;
use deno_core::ErrBox;
use futures::Future;

mod compiler_worker;
mod js;
mod json;
mod ts;
mod wasm;

pub use js::JsCompiler;
pub use json::JsonCompiler;
pub use ts::runtime_compile;
pub use ts::runtime_transpile;
pub use ts::TargetLib;
pub use ts::TsCompiler;
pub use wasm::WasmCompiler;

pub type CompilationResultFuture = dyn Future<Output = JsonResult>;

#[derive(Debug, Clone)]
pub struct CompiledModule {
  pub code: String,
  pub name: String,
}

pub type CompiledModuleFuture =
  dyn Future<Output = Result<CompiledModule, ErrBox>>;
