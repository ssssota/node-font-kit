use std::collections::HashSet;
use core_text::font_collection::create_for_all_families;

pub fn get_all() -> HashSet<String> {
    create_for_all_families()
        .get_descriptors()
        .unwrap()
        .iter()
        .filter_map(|d| d.font_path())
        .filter_map(|p| p.to_str())
        .map(|p| p.to_string())
        .collect()
}
