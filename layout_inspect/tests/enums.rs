use std::mem::{align_of, size_of, transmute};

use layout_inspect::{
	defs::{DefEnum, DefEnumVariant, DefType},
	inspect, Inspectable,
};

#[test]
fn enum_fieldless() {
	#[derive(Inspectable)]
	#[allow(dead_code)]
	enum Foo {
		/// `o1`
		Opt1,
		/// `o2`
		Opt2,
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Enum(DefEnum {
			name: "Foo".to_string(),
			size: 1,
			align: 1,
			variants: vec![
				DefEnumVariant {
					name: "Opt1".to_string(),
					discriminant: 0,
					ser_value: Some("o1".to_string()),
					value_type_id: None
				},
				DefEnumVariant {
					name: "Opt2".to_string(),
					discriminant: 1,
					ser_value: Some("o2".to_string()),
					value_type_id: None
				},
			],
		})
	);

	// Check discriminants are correct
	fn to_u8(foo: Foo) -> u8 {
		unsafe { transmute(foo) }
	}
	assert_eq!(to_u8(Foo::Opt1), 0);
	assert_eq!(to_u8(Foo::Opt2), 1);
}

#[test]
fn enum_fieldless_with_discriminants() {
	#[derive(Inspectable)]
	#[allow(dead_code)]
	enum Foo {
		/// `o1`
		Opt1 = 5,
		/// `o2`
		Opt2 = 10,
		/// `o3`
		Opt3,
		/// `o4`
		Opt4,
	}

	assert_eq!(
		inspect::<Foo>()[0],
		DefType::Enum(DefEnum {
			name: "Foo".to_string(),
			size: 1,
			align: 1,
			variants: vec![
				DefEnumVariant {
					name: "Opt1".to_string(),
					discriminant: 5,
					ser_value: Some("o1".to_string()),
					value_type_id: None
				},
				DefEnumVariant {
					name: "Opt2".to_string(),
					discriminant: 10,
					ser_value: Some("o2".to_string()),
					value_type_id: None
				},
				DefEnumVariant {
					name: "Opt3".to_string(),
					discriminant: 11,
					ser_value: Some("o3".to_string()),
					value_type_id: None
				},
				DefEnumVariant {
					name: "Opt4".to_string(),
					discriminant: 12,
					ser_value: Some("o4".to_string()),
					value_type_id: None
				},
			],
		})
	);

	// Check discriminants are correct
	fn to_u8(foo: Foo) -> u8 {
		unsafe { transmute(foo) }
	}
	assert_eq!(to_u8(Foo::Opt1), 5);
	assert_eq!(to_u8(Foo::Opt2), 10);
	assert_eq!(to_u8(Foo::Opt3), 11);
	assert_eq!(to_u8(Foo::Opt4), 12);
}

#[test]
fn enum_fieldful() {
	#[derive(Inspectable)]
	#[allow(dead_code)]
	enum Foo {
		Opt1(u8),
		Opt2(u16),
	}

	let type_defs = inspect::<Foo>();

	assert_eq!(
		&type_defs[0],
		&DefType::Enum(DefEnum {
			name: "Foo".to_string(),
			size: size_of::<Foo>(),
			align: align_of::<Foo>(),
			variants: vec![
				DefEnumVariant {
					name: "Opt1".to_string(),
					discriminant: 0,
					ser_value: None,
					value_type_id: Some(1)
				},
				DefEnumVariant {
					name: "Opt2".to_string(),
					discriminant: 1,
					ser_value: None,
					value_type_id: Some(2)
				},
			],
		})
	);

	let variant_ids = get_variant_ids(&type_defs[0]);
	assert_eq!(type_defs[variant_ids[0].unwrap()].name(), "u8");
	assert_eq!(type_defs[variant_ids[1].unwrap()].name(), "u16");
}

#[test]
fn enum_mixed_fieldless_and_fieldful() {
	#[derive(Inspectable)]
	#[allow(dead_code)]
	enum Foo {
		/// `o1`
		Opt1,
		Opt2(u8),
	}

	let type_defs = inspect::<Foo>();

	assert_eq!(
		&type_defs[0],
		&DefType::Enum(DefEnum {
			name: "Foo".to_string(),
			size: 2,
			align: 1,
			variants: vec![
				DefEnumVariant {
					name: "Opt1".to_string(),
					discriminant: 0,
					ser_value: Some("o1".to_string()),
					value_type_id: None
				},
				DefEnumVariant {
					name: "Opt2".to_string(),
					discriminant: 1,
					ser_value: None,
					value_type_id: Some(1)
				},
			],
		})
	);

	let variant_ids = get_variant_ids(&type_defs[0]);
	assert_eq!(type_defs[variant_ids[1].unwrap()].name(), "u8");

	// Check discriminants are correct
	fn to_bytes(foo: Foo) -> [u8; 2] {
		unsafe { transmute(foo) }
	}
	assert_eq!(to_bytes(Foo::Opt1), [0, 0]);
	assert_eq!(to_bytes(Foo::Opt2(10)), [1, 10]);
}

fn get_variant_ids(enum_def: &DefType) -> Vec<Option<usize>> {
	enum_def
		.to_enum()
		.unwrap()
		.variants
		.iter()
		.map(|variant| variant.value_type_id.map(|id| id as usize))
		.collect()
}
