use crate::font::JsFont;
use font_kit::handle::Handle;
use napi::bindgen_prelude::Uint8Array;
use napi_derive::napi;
use std::sync::Arc;

#[napi(js_name = "Handle")]
pub struct JsHandle {
  handle: Handle,
}

#[napi]
impl JsHandle {
  pub fn from_raw(handle: Handle) -> Self {
    JsHandle { handle }
  }

  #[napi(factory)]
  pub fn from_path(path: String, font_index: u32) -> Self {
    JsHandle {
      handle: Handle::from_path(path.into(), font_index),
    }
  }
  #[napi(factory)]
  pub fn from_memory(bytes: Uint8Array, font_index: u32) -> Self {
    JsHandle {
      handle: Handle::from_memory(Arc::new((*bytes).into()), font_index),
    }
  }
  #[napi]
  pub fn load(&self) -> JsFont {
    JsFont::from_raw(self.handle.load().unwrap())
  }

  #[napi(getter)]
  pub fn path(&self) -> Option<String> {
    match &self.handle {
      Handle::Path {
        path,
        font_index: _,
      } => Some(path.to_str().unwrap().to_string()),
      Handle::Memory {
        bytes: _,
        font_index: _,
      } => None,
    }
  }

  #[napi(getter)]
  pub fn font_index(&self) -> u32 {
    match &self.handle {
      Handle::Path {
        path: _,
        font_index,
      } => *font_index,
      Handle::Memory {
        bytes: _,
        font_index,
      } => *font_index,
    }
  }
}
