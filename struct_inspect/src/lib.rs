use std::collections::hash_map::HashMap;

// Used by `Inspect` derive macro
#[doc(hidden)]
pub use memoffset::offset_of as __offset_of;
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
    fn kind() -> String;
    fn size() -> usize;
    fn align() -> usize;
    fn json() -> Option<String> {
        None
    }
    fn collect_child_types(_types: &mut HashMap<String, String>) -> () {}

    // Should not be overidden
    fn json_full() -> String {
        format!(
            "{{\"name\":\"{}\",\"kind\":\"{}\",\"size\":{},\"align\":{}{}}}",
            &Self::name(),
            &Self::kind(),
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
}
