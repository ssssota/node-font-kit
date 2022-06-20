use font_kit::family_name::FamilyName;

pub fn str2family_name(name: &str) -> FamilyName {
  match name {
    "serif" => FamilyName::Serif,
    "sans-serif" => FamilyName::SansSerif,
    "monospace" => FamilyName::Monospace,
    "cursive" => FamilyName::Cursive,
    "fantasy" => FamilyName::Fantasy,
    _ => FamilyName::Title(name.to_string()),
  }
}
