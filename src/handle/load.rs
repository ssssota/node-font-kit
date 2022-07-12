use font_kit::handle::Handle;
use napi::{Error, Result, Task};

use crate::font::JsFont;

pub struct Load<'a> {
  pub handle: &'a Handle,
}

impl<'a> Load<'a> {
  pub fn new(handle: &'a Handle) -> Self {
    Load { handle }
  }
}

impl Task for Load<'_> {
  type Output = ();
  type JsValue = JsFont;

  fn compute(&mut self) -> Result<Self::Output> {
    Ok(())
  }

  fn resolve(&mut self, _env: napi::Env, _output: Self::Output) -> Result<Self::JsValue> {
    let font = self
      .handle
      .load()
      .map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(JsFont::from(font))
  }
}
