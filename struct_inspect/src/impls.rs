use std::mem::{align_of, size_of};

use crate::{
	defs::{DefBox, DefOption, DefString, DefType, DefVec},
	Inspect, TypesCollector,
};

impl Inspect for String {
	fn name() -> String {
		"String".to_string()
	}

	fn def(_collector: &mut TypesCollector) -> DefType {
		DefType::String(DefString {
			name: Self::name(),
			size: size_of::<Self>(),
			align: align_of::<Self>(),
		})
	}
}

macro_rules! single_type_param {
	($name:ident, $def:ident) => {
		impl<T: Inspect> Inspect for $name<T> {
			fn name() -> String {
				stringify!($name).to_string() + "<" + &T::name() + ">"
			}

			fn def(collector: &mut TypesCollector) -> DefType {
				DefType::$name($def {
					name: Self::name(),
					size: size_of::<Self>(),
					align: align_of::<Self>(),
					value_type_id: collector.collect::<T>(),
				})
			}
		}
	};
}

single_type_param!(Box, DefBox);
single_type_param!(Vec, DefVec);
single_type_param!(Option, DefOption);
