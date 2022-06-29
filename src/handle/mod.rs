use font_kit::handle::Handle;
use napi::bindgen_prelude::{AsyncTask, Error, Result, Uint8Array};
use napi_derive::napi;
use std::sync::Arc;

mod load;

use load::Load;

/// Encapsulates the information needed to locate and open a font.
///
/// This is either the path to the font or the raw in-memory font data.
///
/// To open the font referenced by a handle, use a loader.
///
/// ref. [Handle](https://docs.rs/font-kit/latest/font_kit/handle/enum.Handle.html)
#[napi(js_name = "Handle")]
pub struct JsHandle {
  handle: Handle,
}

#[napi]
impl JsHandle {
  /// Creates a new handle from a path.
  ///
  /// font_index specifies the index of the font to choose if the path points to a font collection. If the path points to a single font file, pass 0.
  ///
  /// ref. [from_path](https://docs.rs/font-kit/latest/font_kit/handle/enum.Handle.html#method.from_path)
  #[napi(factory)]
  pub fn from_path(path: String, font_index: u32) -> Self {
    JsHandle {
      handle: Handle::from_path(path.into(), font_index),
    }
  }

  /// Creates a new handle from raw TTF/OTF/etc. data in memory.
  ///
  /// font_index specifies the index of the font to choose if the memory represents a font collection. If the memory represents a single font file, pass 0.
  ///
  /// ref. [from_memory](https://docs.rs/font-kit/latest/font_kit/handle/enum.Handle.html#method.from_memory)
  #[napi(factory)]
  pub fn from_memory(bytes: Uint8Array, font_index: u32) -> Self {
    JsHandle {
      handle: Handle::from_memory(Arc::new((*bytes).into()), font_index),
    }
  }

  /// A convenience method to load this handle with the default loader, producing a Font.
  ///
  /// ref. [load](https://docs.rs/font-kit/latest/font_kit/handle/enum.Handle.html#method.load)
  #[napi(ts_return_type = "Promise<Font>")]
  pub fn load(&self) -> AsyncTask<Load> {
    AsyncTask::new(Load::new(&self.handle))
  }

  /// The path to the font.
  ///
  /// ref. [path](https://docs.rs/font-kit/latest/font_kit/handle/enum.Handle.html#variant.Path.field.path)
  #[napi(getter)]
  pub fn path(&self) -> Result<String> {
    match &self.handle {
      Handle::Path {
        path,
        font_index: _,
      } => Ok(
        path
          .to_str()
          .ok_or_else(|| Error::from_reason("Failed to convert from OS string"))?
          .to_string(),
      ),
      Handle::Memory {
        bytes: _,
        font_index: _,
      } => Err(Error::from_reason("Handle is in memory")),
    }
  }

  /// The index of the font, if the path refers to a collection.
  ///
  /// If the path refers to a single font, this value will be 0.
  ///
  /// ref. [font_index](https://docs.rs/font-kit/latest/font_kit/handle/enum.Handle.html#variant.Path.field.font_index)
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

impl From<JsHandle> for Handle {
  #[inline]
  fn from(js: JsHandle) -> Self {
    js.handle
  }
}

impl From<Handle> for JsHandle {
  #[inline]
  fn from(handle: Handle) -> Self {
    JsHandle { handle }
  }
}
