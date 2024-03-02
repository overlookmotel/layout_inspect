use std::mem::{align_of, size_of};

use layout_inspect::{
	defs::{
		DefPhantomData, DefResult, DefStr, DefStrSlice, DefString, DefStruct, DefStructField, DefType,
	},
	inspect, Inspect,
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

#[test]
fn str() {
	assert_eq!(
		inspect::<str>()[0],
		DefType::Str(DefStr {
			name: "str".to_string(),
			size: None,
			align: 1,
		})
	);
}

#[test]
fn str_slice() {
	assert_eq!(
		inspect::<&str>()[0],
		DefType::StrSlice(DefStrSlice {
			name: "&str".to_string(),
			size: size_of::<&str>(),
			align: align_of::<&str>(),
		})
	);

	#[derive(Inspect)]
	struct Foo<'a> {
		s: &'a str,
		s2: &'static str,
	}

	let type_defs = inspect::<Foo>();

	assert_eq!(
		&type_defs[0],
		&DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			ser_name: "Foo".to_string(),
			size: Some(size_of::<Foo>()),
			align: Some(align_of::<Foo>()),
			fields: vec![
				DefStructField {
					name: "s".to_string(),
					ser_name: "s".to_string(),
					type_id: 1,
					offset: 0,
					flatten: false,
					skip: false,
				},
				DefStructField {
					name: "s2".to_string(),
					ser_name: "s2".to_string(),
					type_id: 1,
					offset: 16,
					flatten: false,
					skip: false,
				}
			],
			tag: None,
			transparent: false,
		})
	);

	assert_eq!(
		&type_defs[1],
		&DefType::StrSlice(DefStrSlice {
			name: "&str".to_string(),
			size: size_of::<&str>(),
			align: align_of::<&str>(),
		})
	);
}

#[test]
fn phantom_data() {
	use std::marker::PhantomData;

	let type_defs = inspect::<PhantomData<u128>>();

	assert_eq!(
		&type_defs[0],
		&DefType::PhantomData(DefPhantomData {
			name: "PhantomData<u128>".to_string(),
			size: 0,
			align: 1,
			value_type_id: 1,
		})
	);

	assert_eq!(type_defs[1].name(), "u128");
}

#[test]
fn result() {
	let type_defs = inspect::<Result<u8, u16>>();

	assert_eq!(
		&type_defs[0],
		&DefType::Result(DefResult {
			name: "Result<u8,u16>".to_string(),
			size: size_of::<Result<u8, u16>>(),
			align: align_of::<Result<u8, u16>>(),
			ok_type_id: 1,
			err_type_id: 2,
		})
	);

	assert_eq!(type_defs[1].name(), "u8");
	assert_eq!(type_defs[2].name(), "u16");
}
