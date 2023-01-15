use std::{collections::hash_map::HashMap, fs, io, path::Path};

// Used by `Inspect` derive macro
#[doc(hidden)]
pub use memoffset::offset_of as __offset_of;
use serde_json;
pub use struct_inspect_derive::Inspect;

pub mod defs;
mod impls;
mod primitives;
use defs::DefType;

pub fn inspect<T: Inspect>() -> HashMap<String, DefType> {
	let mut types = HashMap::<String, DefType>::new();
	T::collect_types(&mut types);
	types
}

pub fn write_types_to_json_file<P: AsRef<Path>>(
	types: &HashMap<String, DefType>,
	path: P,
	pretty: bool,
) -> io::Result<()> {
	let json = types_to_json(&types, pretty);
	fs::write(path, json)
}

pub fn types_to_json(types: &HashMap<String, DefType>, pretty: bool) -> String {
	if pretty {
		serde_json::to_string_pretty(types).unwrap()
	} else {
		serde_json::to_string(types).unwrap()
	}
}

pub trait Inspect {
	// To be defined in impls
	fn name() -> String;
	fn def() -> DefType;
	fn collect_child_types(_types: &mut HashMap<String, DefType>) {}

	// Should not be overidden
	fn collect_types(types: &mut HashMap<String, DefType>) -> () {
		let name = Self::name();
		if types.contains_key(&name) {
			return;
		};
		types.insert(name, Self::def());
		Self::collect_child_types(types);
	}
}
