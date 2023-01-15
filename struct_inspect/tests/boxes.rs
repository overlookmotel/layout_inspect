use std::mem::{align_of, size_of};

use struct_inspect::{
	defs::{DefBox, DefType},
	Inspect,
};

#[test]
fn boxed_primitive() {
	assert_eq!(Box::<u8>::name(), "Box<U8>");
	assert_eq!(
		Box::<u8>::def(),
		DefType::Box(DefBox {
			name: "Box<U8>".to_string(),
			size: size_of::<usize>(),
			align: align_of::<usize>(),
			value_type_name: "U8".to_string(),
		})
	);
}

#[test]
fn boxed_struct() {
	#[derive(Inspect)]
	struct Foo {
		small: u8,
		big: u128,
	}

	assert_eq!(Box::<Foo>::name(), "Box<Foo>");
	assert_eq!(
		Box::<Foo>::def(),
		DefType::Box(DefBox {
			name: "Box<Foo>".to_string(),
			size: size_of::<usize>(),
			align: align_of::<usize>(),
			value_type_name: "Foo".to_string(),
		})
	);
}
