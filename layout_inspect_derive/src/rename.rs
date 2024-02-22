use convert_case::{Case, Casing};
use syn::{AttrStyle, Attribute, Lit, Meta, NestedMeta};

/// Find `#[serde(rename)]` or `#[serde(rename_all)]` attributes
pub fn get_rename_attrs(attrs: &Vec<Attribute>) -> (Option<String>, Option<String>) {
	let mut rename = None;
	let mut rename_all = None;
	for attr in attrs {
		if attr.style == AttrStyle::Outer && attr.path.is_ident("serde") {
			let meta = attr.parse_meta().unwrap();
			if let Meta::List(list) = meta {
				for item in list.nested {
					if let NestedMeta::Meta(Meta::NameValue(name_value)) = item {
						if name_value.path.is_ident("rename") {
							if let Lit::Str(s) = name_value.lit {
								if rename.is_some() {
									panic!("Multiple serde rename tags on same struct/enum");
								}
								rename = Some(s.value());
							} else {
								panic!("Unexpected serde rename tag");
							}
						} else if name_value.path.is_ident("rename_all") {
							if let Lit::Str(s) = name_value.lit {
								if rename_all.is_some() {
									panic!("Multiple serde rename_all tags on same struct/enum");
								}
								rename_all = Some(s.value());
							} else {
								panic!("Unexpected serde rename_all tag");
							}
						}
					}
				}
			}
		}
	}
	(rename, rename_all)
}

/// Rename field/varient name according to `#[serde(rename_all = "...")]`
/// attribute
pub fn rename(s: &str, attr: &str) -> String {
	match attr {
		"lowercase" => s.to_lowercase(),
		"UPPERCASE" => s.to_uppercase(),
		"PascalCase" => s.to_case(Case::Pascal),
		"camelCase" => s.to_case(Case::Camel),
		"snake_case" => s.to_case(Case::Snake),
		"SCREAMING_SNAKE_CASE" => s.to_case(Case::UpperSnake),
		"kebab-case" => s.to_case(Case::Kebab),
		"SCREAMING-KEBAB-CASE" => s.to_case(Case::UpperKebab),
		_ => panic!("Unexpected `serde(rename_all)` attribute `{}`", attr),
	}
}
