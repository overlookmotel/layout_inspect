use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind")]
pub enum DefType {
    Struct(DefStruct),
    Primitive(DefPrimitive),
    Box(DefBox),
    Vec(DefVec),
    Option(DefOption),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefStruct {
    pub name: String,
    pub size: usize,
    pub align: usize,
    pub fields: Vec<DefStructField>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefStructField {
    pub name: String,
    pub type_name: String,
    pub offset: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefPrimitive {
    pub name: String,
    pub size: usize,
    pub align: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefBox {
    pub name: String,
    pub size: usize,
    pub align: usize,
    pub value_type_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefVec {
    pub name: String,
    pub size: usize,
    pub align: usize,
    pub value_type_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefOption {
    pub name: String,
    pub size: usize,
    pub align: usize,
    pub value_type_name: String,
}
