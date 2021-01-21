use neon::prelude::*;

mod path;
use path::get_path_all;

register_module!(mut cx, {
    cx.export_function("getPathAll", get_path_all)?;
    Ok(())
});
