#![cfg_attr(feature = "nightly", feature(core_intrinsics))]

use std::collections::hash_map::HashMap;

pub use layout_inspect_derive::Inspect;
// Used by `Inspect` derive macro
#[doc(hidden)]
pub use memoffset::offset_of as __offset_of;

pub mod defs;
mod impls;
mod primitives;
use defs::DefType;

pub type TypeId = u32;

pub fn inspect<T: Inspect + ?Sized>() -> Vec<DefType> {
	let mut collector = TypesCollector::new();
	collector.collect::<T>();
	collector.into_types()
}

// `'static` bound required by `any::TypeId::of()`
pub trait Inspect: 'static {
	fn name() -> String;
	fn size() -> Option<usize>;
	fn align() -> Option<usize>;
	fn def(collector: &mut TypesCollector) -> DefType;
}

pub struct TypesCollector {
	types: Vec<Option<DefType>>,
	native_type_id_to_id: HashMap<u64, TypeId>,
}

impl TypesCollector {
	fn new() -> Self {
		TypesCollector {
			types: Vec::new(),
			native_type_id_to_id: HashMap::new(),
		}
	}

	pub fn collect<T: Inspect + ?Sized>(&mut self) -> TypeId {
		let native_id = type_id_of::<T>();

		if let Some(id) = self.native_type_id_to_id.get(&native_id) {
			*id
		} else {
			let id = self.types.len() as TypeId;
			self.native_type_id_to_id.insert(native_id, id);

			self.types.push(None);

			let def = T::def(self);
			self.types[id as usize] = Some(def);
			id
		}
	}

	fn into_types(self) -> Vec<DefType> {
		self
			.types
			.into_iter()
			.map(|type_def| type_def.unwrap())
			.collect()
	}
}

#[cfg(all(feature = "stable", feature = "nightly"))]
compile_error!("stable and nightly features cannot both be enabled");

#[cfg(not(any(feature = "stable", feature = "nightly")))]
compile_error!("either stable or nightly feature must be enabled");

#[cfg(feature = "nightly")]
fn type_id_of<T: 'static + ?Sized>() -> u64 {
	use std::intrinsics::type_id;
	type_id::<T>()
}

#[cfg(all(feature = "stable", not(feature = "nightly")))]
fn type_id_of<T: 'static + ?Sized>() -> u64 {
	// Hacky way to get Rust's native type ID without nightly.
	// `std::any::TypeId` does not expose any direct way to get the actual u64 ID.
	use std::any;

	use regex::Regex;
	let id = format!("{:?}", any::TypeId::of::<T>());
	let regex = Regex::new(r"^TypeId\s*\{\s*t:\s*(\d+)\s*\}$").unwrap();
	regex.captures(&id).unwrap()[1].parse::<u64>().unwrap()
}
