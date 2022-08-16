pub use std::mem;

pub use memoffset::offset_of;
pub use struct_inspect_derive::Inspect;

mod impls;
mod primitives;

pub trait Inspect {
    fn type_def() -> TypeDef;
    fn fields_def() -> Option<Vec<FieldDef>> {
        None
    }
}

#[derive(Debug)]
pub struct TypeDef {
    pub name: String,
    pub kind: TypeKind,
    pub len: usize,
    pub child: Option<Box<TypeDef>>,
}

#[derive(Debug)]
pub enum TypeKind {
    Struct,
    Enum,
    Vec,
    Box,
    Option,
    Primitive,
}

#[derive(Debug)]
pub struct FieldDef {
    pub name: &'static str,
    pub offset: usize,
    pub type_def: TypeDef,
}
