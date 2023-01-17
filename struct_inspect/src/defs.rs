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

impl DefType {
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

	pub fn into_primitive(self) -> Option<DefPrimitive> {
		match self {
			DefType::Primitive(def) => Some(def),
			_ => None,
		}
	}

	pub fn into_struct(self) -> Option<DefStruct> {
		match self {
			DefType::Struct(def) => Some(def),
			_ => None,
		}
	}

	pub fn into_enum(self) -> Option<DefEnum> {
		match self {
			DefType::Enum(def) => Some(def),
			_ => None,
		}
	}

	pub fn into_box(self) -> Option<DefBox> {
		match self {
			DefType::Box(def) => Some(def),
			_ => None,
		}
	}

	pub fn into_vec(self) -> Option<DefVec> {
		match self {
			DefType::Vec(def) => Some(def),
			_ => None,
		}
	}

	pub fn into_option(self) -> Option<DefOption> {
		match self {
			DefType::Option(def) => Some(def),
			_ => None,
		}
	}

	pub fn into_string(self) -> Option<DefString> {
		match self {
			DefType::String(def) => Some(def),
			_ => None,
		}
	}

	pub fn primitive_ref(&self) -> Option<&DefPrimitive> {
		match self {
			DefType::Primitive(def) => Some(def),
			_ => None,
		}
	}

	pub fn struct_ref(&self) -> Option<&DefStruct> {
		match self {
			DefType::Struct(def) => Some(def),
			_ => None,
		}
	}

	pub fn enum_ref(&self) -> Option<&DefEnum> {
		match self {
			DefType::Enum(def) => Some(def),
			_ => None,
		}
	}

	pub fn box_ref(&self) -> Option<&DefBox> {
		match self {
			DefType::Box(def) => Some(def),
			_ => None,
		}
	}

	pub fn vec_ref(&self) -> Option<&DefVec> {
		match self {
			DefType::Vec(def) => Some(def),
			_ => None,
		}
	}

	pub fn option_ref(&self) -> Option<&DefOption> {
		match self {
			DefType::Option(def) => Some(def),
			_ => None,
		}
	}

	pub fn string_ref(&self) -> Option<&DefString> {
		match self {
			DefType::String(def) => Some(def),
			_ => None,
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
	pub discriminant: u64, // TODO `u32` would be sufficient, or use `usize`
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
