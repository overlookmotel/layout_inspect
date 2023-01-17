use std::mem::{align_of, size_of};

use struct_inspect::{
	defs::{DefString, DefType},
	inspect,
};

#[test]
fn string() {
	assert_eq!(
		inspect::<String>()[0],
		DefType::String(DefString {
			name: "String".to_string(),
			size: size_of::<String>(),
			align: align_of::<String>(),
		})
	);
}
