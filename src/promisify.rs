use napi::bindgen_prelude::{AsyncTask, ToNapiValue, TypeName};
use napi::{Error, Task};

pub struct Promise<T> {
  result: Result<T, String>,
}

impl<T> Task for Promise<T>
where
  T: Send + Clone + ToNapiValue + TypeName,
{
  type Output = ();
  type JsValue = T;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    Ok(())
  }
  fn resolve(&mut self, _env: napi::Env, _outputt: Self::Output) -> napi::Result<Self::JsValue> {
    match &self.result {
      Ok(val) => Ok(val.clone()),
      Err(err) => Err(Error::from_reason(err)),
    }
  }
}

pub fn promisify<T>(result: Result<T, String>) -> AsyncTask<Promise<T>>
where
  T: Send + Clone + ToNapiValue + TypeName,
{
  AsyncTask::new(Promise { result })
}
