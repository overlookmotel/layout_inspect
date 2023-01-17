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
	Enum(DefEnum),
	String(DefString),
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
	to_methods!(Primitive, DefPrimitive, into_primitive, to_primitive);

	to_methods!(Struct, DefStruct, into_struct, to_struct);

	to_methods!(Enum, DefEnum, into_enum, to_enum);

	to_methods!(Box, DefBox, into_box, to_box);

	to_methods!(Vec, DefVec, into_vec, to_vec);

	to_methods!(Option, DefOption, into_option, to_option);

	to_methods!(String, DefString, into_string, to_string);

	pub fn name(&self) -> &str {
		match &self {
			DefType::Struct(DefStruct { name, .. }) => &name[..],
			DefType::Primitive(DefPrimitive { name, .. }) => &name[..],
			DefType::Box(DefBox { name, .. }) => &name[..],
			DefType::Vec(DefVec { name, .. }) => &name[..],
			DefType::Option(DefOption { name, .. }) => &name[..],
			DefType::Enum(DefEnum { name, .. }) => &name[..],
			DefType::String(DefString { name, .. }) => &name[..],
		}
	}

	pub fn size(&self) -> usize {
		match &self {
			DefType::Struct(DefStruct { size, .. }) => *size,
			DefType::Primitive(DefPrimitive { size, .. }) => *size,
			DefType::Box(DefBox { size, .. }) => *size,
			DefType::Vec(DefVec { size, .. }) => *size,
			DefType::Option(DefOption { size, .. }) => *size,
			DefType::Enum(DefEnum { size, .. }) => *size,
			DefType::String(DefString { size, .. }) => *size,
		}
	}

	pub fn align(&self) -> usize {
		match &self {
			DefType::Struct(DefStruct { align, .. }) => *align,
			DefType::Primitive(DefPrimitive { align, .. }) => *align,
			DefType::Box(DefBox { align, .. }) => *align,
			DefType::Vec(DefVec { align, .. }) => *align,
			DefType::Option(DefOption { align, .. }) => *align,
			DefType::Enum(DefEnum { align, .. }) => *align,
			DefType::String(DefString { align, .. }) => *align,
		}
	}
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefString {
	pub name: String,
	pub size: usize,
	pub align: usize,
}
