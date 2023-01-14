use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "kind")]
pub enum DefType {
    Struct(DefStruct),
    Primitive(DefPrimitive),
    Box(DefBox),
    Vec(DefVec),
    Option(DefOption),
    Enum(DefEnum),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefStruct {
    pub name: String,
    pub size: usize,
    pub align: usize,
    pub fields: Vec<DefStructField>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefStructField {
    pub name: String,
    pub js_name: String,
    pub type_name: String,
    pub offset: usize,
    pub flatten: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefPrimitive {
    pub name: String,
    pub size: usize,
    pub align: usize,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefBox {
    pub name: String,
    pub size: usize,
    pub align: usize,
    pub value_type_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefVec {
    pub name: String,
    pub size: usize,
    pub align: usize,
    pub value_type_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefOption {
    pub name: String,
    pub size: usize,
    pub align: usize,
    pub value_type_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefEnum {
    pub name: String,
    pub size: usize,
    pub align: usize,
    pub variants: Vec<DefEnumVariant>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefEnumVariant {
    pub name: String,
    pub discriminant: u64,
    pub value: Option<String>,
    pub value_type_name: Option<String>,
}
