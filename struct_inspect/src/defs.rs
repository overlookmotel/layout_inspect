use serde::{Deserialize, Serialize};

use super::TypeId;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(tag = "kind")]
pub enum DefType {
	Struct(DefStruct),
	Primitive(DefPrimitive),
	Box(DefBox),
	Vec(DefVec),
	Option(DefOption),
	Enum(DefEnum),
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
		}
	}

	pub fn into_struct(self) -> Option<DefStruct> {
		match self {
			DefType::Struct(def) => Some(def),
			_ => None,
		}
	}

	pub fn into_primitive(self) -> Option<DefPrimitive> {
		match self {
			DefType::Primitive(def) => Some(def),
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

	pub fn into_enum(self) -> Option<DefEnum> {
		match self {
			DefType::Enum(def) => Some(def),
			_ => None,
		}
	}

	pub fn struct_ref(&self) -> Option<&DefStruct> {
		match self {
			DefType::Struct(def) => Some(def),
			_ => None,
		}
	}

	pub fn primitive_ref(&self) -> Option<&DefPrimitive> {
		match self {
			DefType::Primitive(def) => Some(def),
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

	pub fn enum_ref(&self) -> Option<&DefEnum> {
		match self {
			DefType::Enum(def) => Some(def),
			_ => None,
		}
	}
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
	pub js_name: String,
	pub type_id: TypeId,
	pub offset: usize,
	pub flatten: bool,
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
pub struct DefBox {
	pub name: String,
	pub size: usize,
	pub align: usize,
	pub value_type_id: TypeId,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefVec {
	pub name: String,
	pub size: usize,
	pub align: usize,
	pub value_type_id: TypeId,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefOption {
	pub name: String,
	pub size: usize,
	pub align: usize,
	pub value_type_id: TypeId,
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
	pub value: Option<String>,
	pub value_type_id: Option<TypeId>,
}
