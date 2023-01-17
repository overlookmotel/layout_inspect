use std::{
	marker::PhantomData,
	mem::{align_of, size_of},
};

use crate::{
	defs::{DefBox, DefOption, DefPhantomData, DefResult, DefString, DefType, DefVec},
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
single_type_param!(PhantomData, DefPhantomData);

macro_rules! double_type_param {
	($name:ident, $def:ident, $field1:ident, $field2:ident) => {
		impl<T: Inspect, T2: Inspect> Inspect for $name<T, T2> {
			fn name() -> String {
				stringify!($name).to_string() + "<" + &T::name() + "," + &T2::name() + ">"
			}

			fn def(collector: &mut TypesCollector) -> DefType {
				DefType::$name($def {
					name: Self::name(),
					size: size_of::<Self>(),
					align: align_of::<Self>(),
					$field1: collector.collect::<T>(),
					$field2: collector.collect::<T2>(),
				})
			}
		}
	};
}

double_type_param!(Result, DefResult, ok_type_id, err_type_id);
