use crate::handle::JsHandle;
use font_kit::family_handle::FamilyHandle;
use napi_derive::napi;

#[napi(js_name = "FamilyHandle")]
pub struct JsFamilyHandle {
  family_handle: FamilyHandle,
}

#[napi]
impl JsFamilyHandle {
  #[napi(constructor)]
  pub fn new() -> Self {
    JsFamilyHandle {
      family_handle: FamilyHandle::new(),
    }
  }

  #[napi]
  pub fn is_empty(&self) -> bool {
    self.family_handle.is_empty()
  }

  #[napi]
  pub fn fonts(&self) -> Vec<JsHandle> {
    self
      .family_handle
      .fonts()
      .iter()
      .map(|handle| JsHandle::from(handle.clone()))
      .collect()
  }
}

impl From<JsFamilyHandle> for FamilyHandle {
  fn from(js: JsFamilyHandle) -> Self {
    js.family_handle
  }
}

impl From<FamilyHandle> for JsFamilyHandle {
  fn from(family_handle: FamilyHandle) -> Self {
    JsFamilyHandle { family_handle }
  }
}

impl Default for JsFamilyHandle {
  fn default() -> Self {
    JsFamilyHandle::new()
  }
}
