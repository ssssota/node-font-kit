use font_kit::{
    file_type::FileType,
    font::Font,
    properties::{Stretch, Style, Weight},
};
use neon::{
    context::{Context, FunctionContext},
    object::Object,
    prelude::Handle,
    result::{JsResult, Throw},
    types::{JsArray, JsBoolean, JsNumber, JsObject, JsString, JsUndefined},
};

struct FontProperties {
    fullname: String,
    family: String,
    postscript_name: String,
    monospace: bool,
    weight: Weight,
    strech: Stretch,
    style: Style,
}

pub fn get_props(mut cx: FunctionContext) -> JsResult<JsArray> {
    let path = cx.argument::<JsString>(0)?.value();
    let size = match Font::analyze_path(&path).unwrap() {
        FileType::Single => 1,
        FileType::Collection(s) => s,
    };
    let array = JsArray::new(&mut cx, size);

    for i in 0..size {
        if let Ok(props) = load_font(&path, i) {
            let res = props_to_object(&mut cx, props);
            match res {
                Ok(obj) => array.set(&mut cx, i, obj)?,
                Err(_) => array.set(&mut cx, i, JsUndefined::new())?,
            };
        } else {
            array.set(&mut cx, i, JsUndefined::new())?;
        }
    }

    Ok(array)
}

fn load_font<P: AsRef<std::path::Path>>(path: P, index: u32) -> Result<FontProperties, Throw> {
    let font = Font::from_path(path, index).map_err(|_| Throw)?;
    let fullname = font.full_name();
    let family = font.family_name();
    let postscript_name = font.postscript_name().unwrap_or("".to_string());
    let monospace = font.is_monospace();
    let props = font.properties();

    Ok(FontProperties {
        fullname,
        family,
        postscript_name,
        monospace,
        weight: props.weight,
        strech: props.stretch,
        style: props.style,
    })
}

fn props_to_object<'a, C: Context<'a>>(
    cx: &mut C,
    props: FontProperties,
) -> Result<Handle<'a, JsObject>, Throw> {
    let object = JsObject::new(cx);

    let fullname = JsString::new(cx, props.fullname);
    let family = JsString::new(cx, props.family);
    let postscript_name = JsString::new(cx, props.postscript_name);
    let monospace = JsBoolean::new(cx, props.monospace);
    let Weight(weight_value) = props.weight;
    let weight = JsNumber::new(cx, weight_value);
    let Stretch(stretch_value) = props.strech;
    let stretch = JsNumber::new(cx, stretch_value);
    let style = JsString::new(
        cx,
        match props.style {
            Style::Normal => "Normal",
            Style::Italic => "Italic",
            Style::Oblique => "Oblique",
        },
    );

    object.set(cx, "fullname", fullname)?;
    object.set(cx, "family", family)?;
    object.set(cx, "postscriptName", postscript_name)?;
    object.set(cx, "monospace", monospace)?;
    object.set(cx, "weight", weight)?;
    object.set(cx, "stretch", stretch)?;
    object.set(cx, "style", style)?;

    Ok(object)
}
