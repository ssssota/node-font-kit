use std::collections::HashSet;
use font_kit::{handle::Handle, source::SystemSource};

pub fn get_all() -> HashSet<String> {
    SystemSource::new()
        .all_fonts()
        .unwrap()
        .iter()
        .filter_map(|handle| match handle {
            Handle::Path { path, .. } => path.to_str(),
            _ => None,
        })
        .map(|path| path.to_string())
        .collect()
}
