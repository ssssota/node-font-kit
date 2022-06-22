use crate::{file_type::JsFileType, properties::Properties};
use font_kit::{font::Font, loader::Loader};
use napi::bindgen_prelude::Uint8Array;
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
  pub fn from_bytes(font_data: Uint8Array, font_index: u32) -> Self {
    let font_data: Arc<Vec<u8>> = Arc::new((*font_data).into());
    JsFont {
      font: Font::from_bytes(font_data, font_index).unwrap(),
    }
  }

  #[napi]
  pub fn analyze_path(path: String) -> JsFileType {
    JsFileType::from(Font::analyze_path(path).unwrap())
  }
  #[napi]
  pub fn analyze_bytes(font_data: Uint8Array) -> JsFileType {
    JsFileType::from(Font::analyze_bytes(Arc::new((*font_data).into())).unwrap())
  }

  #[napi]
  pub fn postscript_name(&self) -> Option<String> {
    self.font.postscript_name()
  }

  #[napi]
  pub fn full_name(&self) -> String {
    self.font.full_name()
  }

  #[napi]
  pub fn family_name(&self) -> String {
    self.font.family_name()
  }

  #[napi]
  pub fn is_monospace(&self) -> bool {
    self.font.is_monospace()
  }

  #[napi]
  pub fn properties(&self) -> Properties {
    Properties::from(self.font.properties())
  }

  #[napi]
  pub fn glyph_for_char(&self, character: String) -> Option<u32> {
    self.font.glyph_for_char(character.chars().next().unwrap())
  }

  #[napi]
  pub fn glyph_by_name(&self, name: String) -> Option<u32> {
    self.font.glyph_by_name(&name)
  }

  #[napi]
  pub fn glyph_count(&self) -> u32 {
    self.font.glyph_count()
  }

  #[napi]
  pub fn load_font_data(&self, table_tag: u32) -> Option<Uint8Array> {
    self
      .font
      .load_font_table(table_tag)
      .map(|t| Uint8Array::new((*t).into()))
  }
}

impl From<JsFont> for Font {
  fn from(js: JsFont) -> Self {
    js.font
  }
}

impl From<Font> for JsFont {
  fn from(font: Font) -> Self {
    JsFont { font }
  }
}
