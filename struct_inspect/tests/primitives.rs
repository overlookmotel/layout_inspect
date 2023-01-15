use std::mem::{align_of, size_of};

use struct_inspect::{
	defs::{DefPrimitive, DefType},
	Inspect,
};

#[test]
fn u8() {
	assert_eq!(u8::name(), "U8");
	assert_eq!(
		u8::def(),
		DefType::Primitive(DefPrimitive {
			name: "U8".to_string(),
			size: size_of::<u8>(),
			align: align_of::<u8>(),
		})
	);
}

#[test]
fn u32() {
	assert_eq!(u32::name(), "U32");
	assert_eq!(
		u32::def(),
		DefType::Primitive(DefPrimitive {
			name: "U32".to_string(),
			size: size_of::<u32>(),
			align: align_of::<u32>(),
		})
	);
}

#[test]
fn u128() {
	assert_eq!(u128::name(), "U128");
	assert_eq!(
		u128::def(),
		DefType::Primitive(DefPrimitive {
			name: "U128".to_string(),
			size: size_of::<u128>(),
			align: align_of::<u128>(),
		})
	);
}

#[test]
fn i8() {
	assert_eq!(i8::name(), "I8");
	assert_eq!(
		i8::def(),
		DefType::Primitive(DefPrimitive {
			name: "I8".to_string(),
			size: size_of::<i8>(),
			align: align_of::<i8>(),
		})
	);
}

#[test]
fn f64() {
	assert_eq!(f64::name(), "F64");
	assert_eq!(
		f64::def(),
		DefType::Primitive(DefPrimitive {
			name: "F64".to_string(),
			size: size_of::<f64>(),
			align: align_of::<f64>(),
		})
	);
}

#[test]
fn usize() {
	assert_eq!(usize::name(), "Usize");
	assert_eq!(
		usize::def(),
		DefType::Primitive(DefPrimitive {
			name: "Usize".to_string(),
			size: size_of::<usize>(),
			align: align_of::<usize>(),
		})
	);
}

#[test]
fn isize() {
	assert_eq!(isize::name(), "Isize");
	assert_eq!(
		isize::def(),
		DefType::Primitive(DefPrimitive {
			name: "Isize".to_string(),
			size: size_of::<isize>(),
			align: align_of::<isize>(),
		})
	);
}

#[test]
fn non_zero_u8() {
	use std::num::NonZeroU8;
	assert_eq!(NonZeroU8::name(), "NonZeroU8");
	assert_eq!(
		NonZeroU8::def(),
		DefType::Primitive(DefPrimitive {
			name: "NonZeroU8".to_string(),
			size: size_of::<NonZeroU8>(),
			align: align_of::<NonZeroU8>(),
		})
	);
}

#[test]
fn bool() {
	assert_eq!(bool::name(), "Bool");
	assert_eq!(
		bool::def(),
		DefType::Primitive(DefPrimitive {
			name: "Bool".to_string(),
			size: size_of::<bool>(),
			align: align_of::<bool>(),
		})
	);
}

#[test]
fn unit() {
	assert_eq!(<() as Inspect>::name(), "Unit");
	assert_eq!(
		<() as Inspect>::def(),
		DefType::Primitive(DefPrimitive {
			name: "Unit".to_string(),
			size: 0,
			align: 1,
		})
	);
}
