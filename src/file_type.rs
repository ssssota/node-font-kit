use font_kit::file_type::FileType;
use napi_derive::napi;

#[napi(js_name = "FileType")]
pub struct JsFileType {
  file_type: FileType,
}

#[napi]
impl JsFileType {
  #[napi(factory)]
  pub fn single() -> Self {
    JsFileType {
      file_type: FileType::Single,
    }
  }
  #[napi(factory)]
  pub fn collection(count: u32) -> Self {
    JsFileType {
      file_type: FileType::Collection(count),
    }
  }

  #[napi(getter)]
  pub fn is_single(&self) -> bool {
    match self.file_type {
      FileType::Single => true,
      FileType::Collection(_) => false,
    }
  }

  #[napi(getter)]
  pub fn count(&self) -> u32 {
    match self.file_type {
      FileType::Single => 1,
      FileType::Collection(n) => n,
    }
  }
}

impl From<JsFileType> for FileType {
  fn from(js: JsFileType) -> Self {
    js.file_type
  }
}

impl From<FileType> for JsFileType {
  fn from(t: FileType) -> Self {
    JsFileType { file_type: t }
  }
}
