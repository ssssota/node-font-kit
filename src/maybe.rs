use napi::Either;

#[inline]
pub fn maybe<T>(val: Option<T>) -> Either<T, ()> {
  match val {
    Some(v) => Either::A(v),
    None => Either::B(()),
  }
}
