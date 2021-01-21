use neon::prelude::*;

#[cfg(not(any(target_os = "macos", target_os = "ios",)))]
mod sys;
#[cfg(not(any(target_os = "macos", target_os = "ios",)))]
use sys::get_all;

#[cfg(any(target_os = "macos", target_os = "ios",))]
mod coretext;
#[cfg(any(target_os = "macos", target_os = "ios",))]
use coretext::get_all;

pub fn get_path_all(mut cx: FunctionContext) -> JsResult<JsArray> {
    let set = get_all();
    let array = JsArray::new(&mut cx, set.len() as u32);

    for (i, pathname) in set.iter().enumerate() {
        let jsname = cx.string(pathname);
        array.set(&mut cx, i as u32, jsname)?;
    }

    Ok(array)
}
