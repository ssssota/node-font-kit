use core_text::font_collection::create_for_all_families;
use std::collections::HashSet;

pub fn get_all() -> HashSet<String> {
    create_for_all_families()
        .get_descriptors()
        .unwrap()
        .iter()
        .filter_map(|d| d.font_path())
        .map(|p| p.to_str().unwrap().to_string())
        .collect()
}
