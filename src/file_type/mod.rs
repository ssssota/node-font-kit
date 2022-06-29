use font_kit::file_type::FileType;
use napi_derive::napi;

/// The type of a font file: either a single font or a TrueType/OpenType collection.
///
/// ref. [FileType](https://docs.rs/font-kit/latest/font_kit/file_type/enum.FileType.html)
#[napi(js_name = "FileType")]
pub struct JsFileType {
  file_type: FileType,
}

#[napi]
impl JsFileType {
  /// The font file represents a single font (.ttf, .otf, .woff, etc.)
  ///
  /// ref. [Single](https://docs.rs/font-kit/latest/font_kit/file_type/enum.FileType.html#variant.Single)
  #[napi(factory)]
  pub fn single() -> Self {
    JsFileType {
      file_type: FileType::Single,
    }
  }

  /// The font file represents a collection of fonts (.ttc, .otc, etc.)
  ///
  /// ref. [Collection](https://docs.rs/font-kit/latest/font_kit/file_type/enum.FileType.html#variant.Collection)
  #[napi(factory)]
  pub fn collection(count: u32) -> Self {
    JsFileType {
      file_type: FileType::Collection(count),
    }
  }

  /// Returns true if file type is single (not collection).
  #[napi(getter)]
  pub fn is_single(&self) -> bool {
    match self.file_type {
      FileType::Single => true,
      FileType::Collection(_) => false,
    }
  }

  /// Returns number of font in the file.
  #[napi(getter)]
  pub fn count(&self) -> u32 {
    match self.file_type {
      FileType::Single => 1,
      FileType::Collection(n) => n,
    }
  }
}

impl From<JsFileType> for FileType {
  #[inline]
  fn from(js: JsFileType) -> Self {
    js.file_type
  }
}

impl From<FileType> for JsFileType {
  #[inline]
  fn from(t: FileType) -> Self {
    JsFileType { file_type: t }
  }
}
