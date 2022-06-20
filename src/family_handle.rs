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
}
