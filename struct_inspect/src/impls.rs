use std::{collections::hash_map::HashMap, mem};

pub use crate::{Inspect, TypeDef, TypeKind};

impl<T: Inspect> Inspect for Box<T> {
    fn name() -> String {
        "Box".to_string() + &T::name()
    }

    fn kind() -> String {
        "box".to_string()
    }

    fn size() -> usize {
        mem::size_of::<Box<T>>()
    }

    fn align() -> usize {
        mem::align_of::<Box<T>>()
    }

    fn json() -> Option<String> {
        Some(format!("\"child\":\"{}\"", &T::name()))
    }

    fn collect_child_types(types: &mut HashMap<String, String>) {
        T::collect_types(types);
    }

    fn type_def() -> TypeDef {
        let child_def = T::type_def();
        TypeDef {
            name: "Box".to_string() + &child_def.name,
            kind: TypeKind::Box,
            len: mem::size_of::<Self>(),
            child: Some(Box::new(child_def)),
        }
    }
}

impl<T: Inspect> Inspect for Vec<T> {
    fn name() -> String {
        "Vec".to_string() + &T::name()
    }

    fn kind() -> String {
        "vec".to_string()
    }

    fn size() -> usize {
        mem::size_of::<Vec<T>>()
    }

    fn align() -> usize {
        mem::align_of::<Vec<T>>()
    }

    fn json() -> Option<String> {
        Some(format!("\"child\":\"{}\"", &T::name()))
    }

    fn collect_child_types(types: &mut HashMap<String, String>) {
        T::collect_types(types);
    }

    fn type_def() -> TypeDef {
        let child_def = T::type_def();
        TypeDef {
            name: "Vec".to_string() + &child_def.name,
            kind: TypeKind::Vec,
            len: mem::size_of::<Self>(),
            child: Some(Box::new(child_def)),
        }
    }
}

impl<T: Inspect> Inspect for Option<T> {
    fn name() -> String {
        "Option".to_string() + &T::name()
    }

    fn kind() -> String {
        "option".to_string()
    }

    fn size() -> usize {
        mem::size_of::<Option<T>>()
    }

    fn align() -> usize {
        mem::align_of::<Option<T>>()
    }

    fn json() -> Option<String> {
        Some(format!("\"child\":\"{}\"", &T::name()))
    }

    fn collect_child_types(types: &mut HashMap<String, String>) {
        T::collect_types(types);
    }

    fn type_def() -> TypeDef {
        let child_def = T::type_def();
        TypeDef {
            name: "Option".to_string() + &child_def.name,
            kind: TypeKind::Option,
            len: mem::size_of::<Self>(),
            child: Some(Box::new(child_def)),
        }
    }
}
