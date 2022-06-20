use font_kit::properties::{Stretch, Style, Weight};
use napi_derive::napi;

const NORMAL: &'static str = "normal";
const ITALIC: &'static str = "italic";
const OBLIQUE: &'static str = "oblique";

#[napi(object)]
pub struct Properties {
  #[napi(ts_type = "'normal' | 'italic' | 'oblique'")]
  pub style: String,
  pub weight: f64,
  pub stretch: f64,
}

impl Into<font_kit::properties::Properties> for Properties {
  fn into(self) -> font_kit::properties::Properties {
    font_kit::properties::Properties {
      style: match &*self.style {
        NORMAL => Style::Normal,
        ITALIC => Style::Italic,
        OBLIQUE => Style::Oblique,
        _ => Style::Normal,
      },
      weight: Weight(self.weight as f32),
      stretch: Stretch(self.stretch as f32),
    }
  }
}

impl From<font_kit::properties::Properties> for Properties {
  fn from(props: font_kit::properties::Properties) -> Self {
    Properties {
      style: match props.style {
        Style::Normal => NORMAL.to_string(),
        Style::Italic => ITALIC.to_string(),
        Style::Oblique => OBLIQUE.to_string(),
      },
      weight: props.weight.0 as f64,
      stretch: props.stretch.0 as f64,
    }
  }
}
