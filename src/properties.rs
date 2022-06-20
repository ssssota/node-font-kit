use font_kit::properties::{Stretch, Style, Weight};
use napi_derive::napi;

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
        "normal" => Style::Normal,
        "italic" => Style::Italic,
        "oblique" => Style::Oblique,
        _ => Style::Normal,
      },
      weight: Weight(self.weight as f32),
      stretch: Stretch(self.stretch as f32),
    }
  }
}
