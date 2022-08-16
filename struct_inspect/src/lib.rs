use std::collections::hash_map::HashMap;

pub use memoffset::offset_of;
pub use struct_inspect_derive::Inspect;

mod impls;
mod primitives;

pub fn inspect<T: Inspect>() -> HashMap<String, String> {
    let mut types = HashMap::<String, String>::new();
    T::collect_types(&mut types);
    types
}

pub trait Inspect {
    // To be defined in impls
    fn name() -> String;
    fn size() -> usize;
    fn align() -> usize;
    fn json() -> Option<String> {
        None
    }
    fn collect_child_types(_types: &mut HashMap<String, String>) -> () {}

    fn json_full() -> String {
        format!(
            "{{\"name\":\"{}\",\"size\":{},\"align\":{}{}}}",
            &Self::name(),
            &Self::size(),
            &Self::align(),
            match Self::json() {
                Some(json) => ",".to_string() + &json,
                None => "".to_string(),
            }
        )
    }

    fn collect_types(types: &mut HashMap<String, String>) -> () {
        let name = Self::name();
        if types.contains_key(&name) {
            return;
        };
        types.insert(name, Self::json_full());
        Self::collect_child_types(types);
    }

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
