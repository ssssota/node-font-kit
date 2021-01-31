use neon::prelude::*;

mod path;
use path::get_path_all;

mod props;
use props::get_props;

register_module!(mut cx, {
    cx.export_function("getPathAll", get_path_all)?;
    cx.export_function("getProps", get_props)?;
    Ok(())
});
