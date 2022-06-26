use crate::family_handle::JsFamilyHandle;
use crate::family_name::str2family_name;
use crate::handle::JsHandle;
use crate::properties::Properties;
use font_kit::family_name::FamilyName;
use font_kit::source::Source;
use font_kit::source::SystemSource;
use napi_derive::napi;
use std::ops::Deref;

/// A database of installed fonts that can be queried.
///
/// ref. [Source](https://docs.rs/font-kit/latest/font_kit/source/trait.Source.html)
#[napi(js_name = "Source")]
pub struct JsSource {
  source: Box<dyn Source>,
}

#[napi]
impl JsSource {
  /// Initialize system default source.
  ///
  /// - Linux: fontconfig
  /// - Windows: direct write
  /// - Mac: core text
  ///
  /// ref. [SystemSource](https://docs.rs/font-kit/latest/font_kit/source/index.html#:~:text=SystemSource)
  #[napi(factory)]
  pub fn system() -> Self {
    JsSource {
      source: Box::new(SystemSource::new()),
    }
  }

  /// Returns paths of all fonts installed on the system.
  ///
  /// ref. [all_fonts](https://docs.rs/font-kit/latest/font_kit/sources/fontconfig/struct.FontconfigSource.html#method.all_fonts)
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

  /// Returns the names of all families installed on the system.
  ///
  /// ref. [all_families](https://docs.rs/font-kit/latest/font_kit/sources/fontconfig/struct.FontconfigSource.html#method.all_families)
  #[napi]
  pub fn all_families(&self) -> Vec<String> {
    self.source.all_families().unwrap()
  }

  /// Looks up a font family by name and returns the handles of all the fonts in that family.
  ///
  /// ref. [select_family_by_name](https://docs.rs/font-kit/latest/font_kit/sources/fontconfig/struct.FontconfigSource.html#method.select_family_by_name)
  #[napi]
  pub fn select_family_by_name(&self, family_name: String) -> JsFamilyHandle {
    JsFamilyHandle::from(self.source.select_family_by_name(&family_name).unwrap())
  }

  /// Selects a font by PostScript name, which should be a unique identifier.
  ///
  /// The default implementation, which is used by the DirectWrite and the filesystem backends, does a brute-force search of installed fonts to find the one that matches.
  ///
  /// ref. [select_by_postscript_name](https://docs.rs/font-kit/latest/font_kit/sources/fontconfig/struct.FontconfigSource.html#method.select_by_postscript_name)
  #[napi]
  pub fn select_by_postscript_name(&self, postscript_name: String) -> JsHandle {
    JsHandle::from(
      self
        .source
        .select_by_postscript_name(&postscript_name)
        .unwrap(),
    )
  }

  /// Performs font matching according to the CSS Fonts Level 3 specification and returns the handle.
  ///
  /// ref. [select_best_match](https://docs.rs/font-kit/latest/font_kit/sources/fontconfig/struct.FontconfigSource.html#method.select_best_match)
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
