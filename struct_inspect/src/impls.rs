use std::{collections::hash_map::HashMap, mem};

pub use crate::Inspect;

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
        Some(format!("\"childType\":\"{}\"", &T::name()))
    }

    fn collect_child_types(types: &mut HashMap<String, String>) {
        T::collect_types(types);
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
        Some(format!("\"childType\":\"{}\"", &T::name()))
    }

    fn collect_child_types(types: &mut HashMap<String, String>) {
        T::collect_types(types);
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
        Some(format!("\"childType\":\"{}\"", &T::name()))
    }

    fn collect_child_types(types: &mut HashMap<String, String>) {
        T::collect_types(types);
    }
}
