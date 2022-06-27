use crate::file_type::JsFileType;
use crate::maybe::maybe;
use crate::properties::Properties;
use font_kit::{font::Font, loader::Loader};
use napi::bindgen_prelude::Uint8Array;
use napi::{Either, Error, Result};
use napi_derive::napi;
use std::sync::Arc;

/// A font face loaded into memory.
///
/// ref. [Font](https://docs.rs/font-kit/latest/font_kit/font/index.html)
#[napi(js_name = "Font")]
pub struct JsFont {
  font: Font,
}

#[napi]
impl JsFont {
  /// Loads a font from the path to a .ttf/.otf/etc. file.
  ///
  /// If the file is a collection (.ttc/.otc/etc.), font_index specifies the index of the font to load from it. If the file represents a single font, pass 0 for font_index.
  ///
  /// ref. [from_path](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.from_path)
  #[napi(factory)]
  pub fn from_path(path: String, font_index: u32) -> Result<Self> {
    let font = Font::from_path(path, font_index).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(JsFont { font })
  }

  /// Loads a font from raw font data (the contents of a .ttf/.otf/etc. file).
  ///
  /// If the data represents a collection (.ttc/.otc/etc.), font_index specifies the index of the font to load from it. If the data represents a single font, pass 0 for font_index.
  ///
  /// ref. [from_bytes](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.from_bytes)
  #[napi(factory)]
  pub fn from_bytes(font_data: Uint8Array, font_index: u32) -> Result<Self> {
    let font_data: Arc<Vec<u8>> = Arc::new((*font_data).into());
    let font =
      Font::from_bytes(font_data, font_index).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(JsFont { font })
  }

  /// Determines whether a path points to a supported font, and, if so, what type of font it is.
  ///
  /// ref. [analyze_path](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.analyze_path)
  #[napi]
  pub fn analyze_path(path: String) -> Result<JsFileType> {
    let file_type = Font::analyze_path(path).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(JsFileType::from(file_type))
  }

  /// Determines whether a blob of raw font data represents a supported font, and, if so, what type of font it is.
  ///
  /// ref. [analyze_bytes](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.analyze_bytes)
  #[napi]
  pub fn analyze_bytes(font_data: Uint8Array) -> Result<JsFileType> {
    let file_type = Font::analyze_bytes(Arc::new((*font_data).into()))
      .map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(JsFileType::from(file_type))
  }

  /// Returns the PostScript name of the font. This should be globally unique.
  ///
  /// ref. [postscript_name](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.postscript_name)
  #[napi]
  pub fn postscript_name(&self) -> Either<String, ()> {
    maybe(self.font.postscript_name())
  }

  /// Returns the full name of the font (also known as “display name” on macOS).
  ///
  /// ref. [full_name](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.full_name)
  #[napi]
  pub fn full_name(&self) -> String {
    self.font.full_name()
  }

  /// Returns the name of the font family.
  ///
  /// ref. [family_name](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.family_name)
  #[napi]
  pub fn family_name(&self) -> String {
    self.font.family_name()
  }

  /// Returns true if and only if the font is monospace (fixed-width).
  ///
  /// ref. [is_monospace](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.is_monospace)
  #[napi]
  pub fn is_monospace(&self) -> bool {
    self.font.is_monospace()
  }

  /// Returns the values of various font properties, corresponding to those defined in CSS.
  ///
  /// ref. [properties](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.properties)
  #[napi]
  pub fn properties(&self) -> Properties {
    Properties::from(self.font.properties())
  }

  /// Returns the usual glyph ID for a Unicode character.
  ///
  /// Be careful with this function; typographically correct character-to-glyph mapping must be done using a shaper such as HarfBuzz. This function is only useful for best-effort simple use cases like “what does character X look like on its own”.
  ///
  /// ref. [glyph_for_char](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.glyph_for_char)
  #[napi]
  pub fn glyph_for_char(&self, character: String) -> Result<Either<u32, ()>> {
    let char = character
      .chars()
      .next()
      .ok_or(Error::from_reason("String is blank"))?;
    Ok(maybe(self.font.glyph_for_char(char)))
  }

  /// Returns the glyph ID for the specified glyph name.
  ///
  /// ref. [glyph_by_name](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.glyph_by_name)
  #[napi]
  pub fn glyph_by_name(&self, name: String) -> Either<u32, ()> {
    maybe(self.font.glyph_by_name(&name))
  }

  /// Returns the number of glyphs in the font.
  ///
  /// Glyph IDs range from 0 inclusive to this value exclusive.
  ///
  /// ref. [glyph_count](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.glyph_count)
  #[napi]
  pub fn glyph_count(&self) -> u32 {
    self.font.glyph_count()
  }

  /// Returns the raw contents of the OpenType table with the given tag.
  ///
  /// Tags are four-character codes. A list of tags can be found in the [OpenType specification](https://docs.microsoft.com/en-us/typography/opentype/spec/).
  ///
  /// ref. [load_font_data](https://docs.rs/font-kit/latest/font_kit/loaders/freetype/struct.Font.html#method.load_font_table)
  #[napi]
  pub fn load_font_data(&self, table_tag: u32) -> Either<Uint8Array, ()> {
    let data = self
      .font
      .load_font_table(table_tag)
      .map(|t| Uint8Array::new((*t).into()));
    maybe(data)
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
