use std::collections::hash_map::HashMap;

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

pub fn types_to_json(types: &HashMap<String, DefType>) -> String {
	serde_json::to_string(types).unwrap()
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
