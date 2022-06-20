use font_kit::font::Font;
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use std::sync::Arc;

#[napi(js_name = "Font")]
pub struct JsFont {
  font: Font,
}

#[napi]
impl JsFont {
  pub fn from_raw(font: Font) -> Self {
    JsFont { font }
  }

  #[napi(factory)]
  pub fn from_path(path: String, font_index: u32) -> Self {
    JsFont {
      font: Font::from_path(path, font_index).unwrap(),
    }
  }
  #[napi(factory)]
  pub fn from_bytes(font_data: Buffer, font_index: u32) -> Self {
    let font_data: Arc<Vec<u8>> = Arc::new((*font_data).to_vec());
    JsFont {
      font: Font::from_bytes(font_data, font_index).unwrap(),
    }
  }
}
