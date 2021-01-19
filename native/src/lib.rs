use font_kit::{handle, sources::fs::*};
use neon::prelude::*;
use std::collections::HashSet;

fn get_path_all(mut cx: FunctionContext) -> JsResult<JsArray> {
    let handles = FsSource::new().all_fonts().unwrap();
    let array = JsArray::new(&mut cx, handles.len() as u32);

    let uniqlist: HashSet<&str> = handles
        .iter()
        .map(|handle| match handle {
            handle::Handle::Path { path, .. } => path.to_str(),
            _ => None,
        })
        .filter(|path| path.is_some())
        .map(|path| path.unwrap())
        .collect();

    for (i, pathname) in uniqlist.iter().enumerate() {
        let jsname = cx.string(pathname);
        array.set(&mut cx, i as u32, jsname)?;
    }

    Ok(array)
}

register_module!(mut cx, {
    cx.export_function("getPathAll", get_path_all)?;
    Ok(())
});
