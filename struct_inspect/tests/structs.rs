use std::mem::{align_of, size_of};

use struct_inspect::{
	defs::{DefStruct, DefStructField, DefType},
	inspect, Inspect,
};

// TODO Test for tuple struct - not implemented yet

#[test]
fn struct_single_field() {
	#[derive(Inspect)]
	struct Foo {
		num: u8,
	}

	let type_defs = inspect::<Foo>();

	assert_eq!(
		&type_defs[0],
		&DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			size: size_of::<Foo>(),
			align: align_of::<Foo>(),
			fields: vec![DefStructField {
				name: "num".to_string(),
				ser_name: "num".to_string(),
				type_id: 1,
				offset: 0,
				flatten: false,
			}]
		})
	);

	let field_ids = get_field_ids(&type_defs[0]);
	assert_eq!(type_defs[field_ids[0]].name(), "u8");
}

#[test]
fn struct_empty() {
	#[derive(Inspect)]
	struct Foo {}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			size: 0,
			align: 1,
			fields: vec![]
		})
	);
}

#[test]
fn struct_multiple_fields() {
	#[derive(Inspect)]
	struct Foo {
		small: u8,
		medium: u16,
		veccy: Vec<u8>,
		recurse: Option<Box<Foo>>,
	}

	let type_defs = inspect::<Foo>();

	assert_eq!(
		&type_defs[0],
		&DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			size: size_of::<Foo>(),
			align: align_of::<Foo>(),
			fields: vec![
				DefStructField {
					name: "small".to_string(),
					ser_name: "small".to_string(),
					type_id: 1,
					offset: size_of::<usize>() * 4 + size_of::<u16>(),
					flatten: false
				},
				DefStructField {
					name: "medium".to_string(),
					ser_name: "medium".to_string(),
					type_id: 2,
					offset: size_of::<usize>() * 4,
					flatten: false
				},
				DefStructField {
					name: "veccy".to_string(),
					ser_name: "veccy".to_string(),
					type_id: 3,
					offset: 0,
					flatten: false
				},
				DefStructField {
					name: "recurse".to_string(),
					ser_name: "recurse".to_string(),
					type_id: 4,
					offset: size_of::<usize>() * 3,
					flatten: false
				}
			]
		})
	);

	let field_ids = get_field_ids(&type_defs[0]);
	assert_eq!(type_defs[field_ids[0]].name(), "u8");
	assert_eq!(type_defs[field_ids[1]].name(), "u16");
	assert_eq!(type_defs[field_ids[2]].name(), "Vec<u8>");
	assert_eq!(type_defs[field_ids[3]].name(), "Option<Box<Foo>>");
}

#[test]
fn struct_generic_one_type_param() {
	#[derive(Inspect)]
	struct Foo {
		big: Bar<u32>,
		small: Bar<u8>,
	}

	#[derive(Inspect)]
	struct Bar<T> {
		inner: T,
	}

	let type_defs = inspect::<Foo>();

	assert_eq!(
		&type_defs[0],
		&DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			size: size_of::<Foo>(),
			align: align_of::<Foo>(),
			fields: vec![
				DefStructField {
					name: "big".to_string(),
					ser_name: "big".to_string(),
					type_id: 1,
					offset: 0,
					flatten: false
				},
				DefStructField {
					name: "small".to_string(),
					ser_name: "small".to_string(),
					type_id: 3,
					offset: size_of::<Bar<u32>>(),
					flatten: false
				}
			]
		})
	);

	let foo_field_ids = get_field_ids(&type_defs[0]);
	let bar_u32_def = &type_defs[foo_field_ids[0]];
	assert_eq!(
		bar_u32_def,
		&DefType::Struct(DefStruct {
			name: "Bar<u32>".to_string(),
			size: size_of::<Bar<u32>>(),
			align: align_of::<Bar<u32>>(),
			fields: vec![DefStructField {
				name: "inner".to_string(),
				ser_name: "inner".to_string(),
				type_id: 2,
				offset: 0,
				flatten: false
			}]
		})
	);

	let u32_id = get_field_ids(bar_u32_def)[0];
	assert_eq!(type_defs[u32_id].name(), "u32");

	let bar_u8_def = &type_defs[foo_field_ids[1]];
	assert_eq!(
		bar_u8_def,
		&DefType::Struct(DefStruct {
			name: "Bar<u8>".to_string(),
			size: size_of::<Bar<u8>>(),
			align: align_of::<Bar<u8>>(),
			fields: vec![DefStructField {
				name: "inner".to_string(),
				ser_name: "inner".to_string(),
				type_id: 4,
				offset: 0,
				flatten: false
			}]
		})
	);

	let u8_id = get_field_ids(bar_u8_def)[0];
	assert_eq!(type_defs[u8_id].name(), "u8");
}

#[test]
fn struct_generic_two_type_params() {
	#[derive(Inspect)]
	struct Foo {
		big: Bar<u64, u32>,
		small: Bar<u16, u8>,
	}

	#[derive(Inspect)]
	struct Bar<T, U> {
		one: T,
		two: U,
	}

	let type_defs = inspect::<Foo>();

	assert_eq!(
		&type_defs[0],
		&DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			size: size_of::<Foo>(),
			align: align_of::<Foo>(),
			fields: vec![
				DefStructField {
					name: "big".to_string(),
					ser_name: "big".to_string(),
					type_id: 1,
					offset: 0,
					flatten: false
				},
				DefStructField {
					name: "small".to_string(),
					ser_name: "small".to_string(),
					type_id: 4,
					offset: size_of::<Bar<u64, u32>>(),
					flatten: false
				}
			]
		})
	);

	let foo_field_ids = get_field_ids(&type_defs[0]);
	let bar_u64_u32_def = &type_defs[foo_field_ids[0]];
	assert_eq!(
		bar_u64_u32_def,
		&DefType::Struct(DefStruct {
			name: "Bar<u64,u32>".to_string(),
			size: size_of::<Bar<u64, u32>>(),
			align: align_of::<Bar<u64, u32>>(),
			fields: vec![
				DefStructField {
					name: "one".to_string(),
					ser_name: "one".to_string(),
					type_id: 2,
					offset: 0,
					flatten: false
				},
				DefStructField {
					name: "two".to_string(),
					ser_name: "two".to_string(),
					type_id: 3,
					offset: size_of::<u64>(),
					flatten: false
				}
			]
		})
	);

	let bar_u64_u32_field_ids = get_field_ids(bar_u64_u32_def);
	assert_eq!(type_defs[bar_u64_u32_field_ids[0]].name(), "u64");
	assert_eq!(type_defs[bar_u64_u32_field_ids[1]].name(), "u32");

	let bar_u16_u8_def = &type_defs[foo_field_ids[1]];
	assert_eq!(
		bar_u16_u8_def,
		&DefType::Struct(DefStruct {
			name: "Bar<u16,u8>".to_string(),
			size: size_of::<Bar<u16, u8>>(),
			align: align_of::<Bar<u16, u8>>(),
			fields: vec![
				DefStructField {
					name: "one".to_string(),
					ser_name: "one".to_string(),
					type_id: 5,
					offset: 0,
					flatten: false
				},
				DefStructField {
					name: "two".to_string(),
					ser_name: "two".to_string(),
					type_id: 6,
					offset: size_of::<u16>(),
					flatten: false
				}
			]
		})
	);

	let bar_u16_u8_field_ids = get_field_ids(bar_u16_u8_def);
	assert_eq!(type_defs[bar_u16_u8_field_ids[0]].name(), "u16");
	assert_eq!(type_defs[bar_u16_u8_field_ids[1]].name(), "u8");
}

#[test]
fn struct_with_serde_field_rename() {
	use serde::{Deserialize, Serialize};

	#[derive(Inspect, Deserialize, Serialize)]
	struct Foo {
		#[serde(rename = "bar")]
		num: u8,
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			size: size_of::<Foo>(),
			align: align_of::<Foo>(),
			fields: vec![DefStructField {
				name: "num".to_string(),
				ser_name: "bar".to_string(),
				type_id: 1,
				offset: 0,
				flatten: false,
			}]
		})
	);
}

#[test]
fn struct_with_serde_field_flatten() {
	use serde::{Deserialize, Serialize};

	#[derive(Inspect, Deserialize, Serialize)]
	struct Foo {
		#[serde(flatten)]
		bar: Bar,
	}

	#[derive(Inspect, Deserialize, Serialize)]
	struct Bar {
		num: u8,
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			size: size_of::<Foo>(),
			align: align_of::<Foo>(),
			fields: vec![DefStructField {
				name: "bar".to_string(),
				ser_name: "bar".to_string(),
				type_id: 1,
				offset: 0,
				flatten: true,
			}]
		})
	);
}

#[test]
fn struct_with_serde_field_rename_and_flatten() {
	use serde::{Deserialize, Serialize};

	#[derive(Inspect, Deserialize, Serialize)]
	struct Foo {
		#[serde(flatten, rename = "qux")]
		bar: Bar,
	}

	#[derive(Inspect, Deserialize, Serialize)]
	struct Bar {
		num: u8,
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			size: size_of::<Foo>(),
			align: align_of::<Foo>(),
			fields: vec![DefStructField {
				name: "bar".to_string(),
				ser_name: "qux".to_string(),
				type_id: 1,
				offset: 0,
				flatten: true,
			}]
		})
	);
}

#[test]
fn struct_with_serde_field_default() {
	use serde::{Deserialize, Serialize};

	#[derive(Inspect, Deserialize, Serialize)]
	struct Foo {
		#[serde(default)]
		num: u8,
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Struct(DefStruct {
			name: "Foo".to_string(),
			size: size_of::<Foo>(),
			align: align_of::<Foo>(),
			fields: vec![DefStructField {
				name: "num".to_string(),
				ser_name: "num".to_string(),
				type_id: 1,
				offset: 0,
				flatten: false,
			}]
		})
	);
}

fn get_field_ids(struct_def: &DefType) -> Vec<usize> {
	struct_def
		.to_struct()
		.unwrap()
		.fields
		.iter()
		.map(|field| field.type_id as usize)
		.collect()
}
