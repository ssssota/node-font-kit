use crate::handle::JsHandle;
use font_kit::family_handle::FamilyHandle;
use napi_derive::napi;

/// Encapsulates the information needed to locate and open the fonts in a family.
///
/// ref. [FamilyHandle](https://docs.rs/font-kit/latest/font_kit/family_handle/struct.FamilyHandle.html)
#[napi(js_name = "FamilyHandle")]
pub struct JsFamilyHandle {
  family_handle: FamilyHandle,
}

#[napi]
impl JsFamilyHandle {
  /// Creates an empty set of family handles.
  ///
  /// ref. [new](https://docs.rs/font-kit/latest/font_kit/family_handle/struct.FamilyHandle.html#method.new)
  #[napi(constructor)]
  pub fn new() -> Self {
    JsFamilyHandle {
      family_handle: FamilyHandle::new(),
    }
  }

  /// Returns true if and only if this set has no fonts in it.
  ///
  /// ref. [is_empty](https://docs.rs/font-kit/latest/font_kit/family_handle/struct.FamilyHandle.html#method.is_empty)
  #[napi]
  pub fn is_empty(&self) -> bool {
    self.family_handle.is_empty()
  }

  /// Returns all the handles in this set.
  ///
  /// ref. [fonts](https://docs.rs/font-kit/latest/font_kit/family_handle/struct.FamilyHandle.html#method.fonts)
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
