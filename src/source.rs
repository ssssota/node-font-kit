use crate::family_handle::JsFamilyHandle;
use crate::family_name::str2family_name;
use crate::handle::JsHandle;
use crate::properties::Properties;
use font_kit::family_name::FamilyName;
use font_kit::source::Source;
use font_kit::source::SystemSource;
use napi_derive::napi;
use std::ops::Deref;

#[napi(js_name = "Source")]
pub struct JsSource {
  source: Box<dyn Source>,
}

#[napi]
impl JsSource {
  #[napi(factory)]
  pub fn system() -> Self {
    JsSource {
      source: Box::new(SystemSource::new()),
    }
  }

  #[napi]
  pub fn all_fonts(&self) -> Vec<JsHandle> {
    self
      .source
      .all_fonts()
      .unwrap()
      .iter()
      .map(|handle| JsHandle::from(handle.clone()))
      .collect()
  }

  #[napi]
  pub fn all_families(&self) -> Vec<String> {
    self.source.all_families().unwrap()
  }

  #[napi]
  pub fn select_family_by_name(&self, family_name: String) -> JsFamilyHandle {
    JsFamilyHandle::from(self.source.select_family_by_name(&family_name).unwrap())
  }

  #[napi]
  pub fn select_by_postscript_name(&self, postscript_name: String) -> JsHandle {
    JsHandle::from(
      self
        .source
        .select_by_postscript_name(&postscript_name)
        .unwrap(),
    )
  }

  #[napi]
  pub fn select_best_match(&self, family_names: Vec<String>, properties: Properties) -> JsHandle {
    JsHandle::from(
      self
        .source
        .select_best_match(
          family_names
            .iter()
            .map(|name| str2family_name(&**name))
            .collect::<Vec<FamilyName>>()
            .deref(),
          &properties.into(),
        )
        .unwrap(),
    )
  }
}
