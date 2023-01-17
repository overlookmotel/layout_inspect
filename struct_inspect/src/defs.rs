use serde::{Deserialize, Serialize};

use super::TypeId;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(tag = "kind")]
#[serde(rename_all = "camelCase")]
pub enum DefType {
	Struct(DefStruct),
	Primitive(DefPrimitive),
	Box(DefBox),
	Vec(DefVec),
	Option(DefOption),
	PhantomData(DefPhantomData),
	Result(DefResult),
	Enum(DefEnum),
	String(DefString),
}

macro_rules! getter {
	($field:ident, $rtn_type:ty, $out:expr) => {
		pub fn $field(&self) -> $rtn_type {
			match &self {
				DefType::Struct(DefStruct { $field, .. }) => $out,
				DefType::Primitive(DefPrimitive { $field, .. }) => $out,
				DefType::Box(DefBox { $field, .. }) => $out,
				DefType::Vec(DefVec { $field, .. }) => $out,
				DefType::Option(DefOption { $field, .. }) => $out,
				DefType::PhantomData(DefPhantomData { $field, .. }) => $out,
				DefType::Result(DefResult { $field, .. }) => $out,
				DefType::Enum(DefEnum { $field, .. }) => $out,
				DefType::String(DefString { $field, .. }) => $out,
			}
		}
	};
}

macro_rules! to_methods {
	($name:ident, $def:ty, $into:ident, $ref:ident) => {
		pub fn $into(self) -> Option<$def> {
			match self {
				DefType::$name(def) => Some(def),
				_ => None,
			}
		}

		pub fn $ref(&self) -> Option<&$def> {
			match self {
				DefType::$name(def) => Some(def),
				_ => None,
			}
		}
	};
}

impl DefType {
	getter!(name, &str, &name[..]);

	getter!(size, usize, *size);

	getter!(align, usize, *align);

	to_methods!(Primitive, DefPrimitive, into_primitive, to_primitive);

	to_methods!(Struct, DefStruct, into_struct, to_struct);

	to_methods!(Enum, DefEnum, into_enum, to_enum);

	to_methods!(Box, DefBox, into_box, to_box);

	to_methods!(Vec, DefVec, into_vec, to_vec);

	to_methods!(Option, DefOption, into_option, to_option);

	to_methods!(
		PhantomData,
		DefPhantomData,
		into_phantom_data,
		to_phantom_data
	);

	to_methods!(Result, DefResult, into_result, to_result);

	to_methods!(String, DefString, into_string, to_string);
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefPrimitive {
	pub name: String,
	pub size: usize,
	pub align: usize,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefStruct {
	pub name: String,
	pub size: usize,
	pub align: usize,
	pub fields: Vec<DefStructField>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefStructField {
	pub name: String,
	pub ser_name: String,
	pub type_id: TypeId,
	pub offset: usize,
	pub flatten: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefEnum {
	pub name: String,
	pub size: usize,
	pub align: usize,
	pub variants: Vec<DefEnumVariant>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefEnumVariant {
	pub name: String,
	pub discriminant: u64,
	pub ser_value: Option<String>,
	pub value_type_id: Option<TypeId>,
}

macro_rules! single_type_param {
	($def:ident) => {
		#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
		#[serde(rename_all = "camelCase")]
		pub struct $def {
			pub name: String,
			pub size: usize,
			pub align: usize,
			pub value_type_id: TypeId,
		}
	};
}

single_type_param!(DefBox);
single_type_param!(DefVec);
single_type_param!(DefOption);
single_type_param!(DefPhantomData);

macro_rules! double_type_param {
	($def:ident, $field1:ident, $field2:ident) => {
		#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
		#[serde(rename_all = "camelCase")]
		pub struct $def {
			pub name: String,
			pub size: usize,
			pub align: usize,
			pub $field1: TypeId,
			pub $field2: TypeId,
		}
	};
}

double_type_param!(DefResult, ok_type_id, err_type_id);

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefString {
	pub name: String,
	pub size: usize,
	pub align: usize,
}
