use std::{collections::hash_map::HashMap, mem};

use crate::{
    defs::{DefBox, DefOption, DefType, DefVec},
    Inspect,
};

impl<T: Inspect> Inspect for Box<T> {
    fn name() -> String {
        "Box<".to_string() + &T::name() + ">"
    }

    fn def() -> DefType {
        DefType::Box(DefBox {
            name: Self::name(),
            size: mem::size_of::<Box<T>>(),
            align: mem::align_of::<Box<T>>(),
            value_type_name: T::name(),
        })
    }

    fn collect_child_types(types: &mut HashMap<String, DefType>) {
        T::collect_types(types);
    }
}

impl<T: Inspect> Inspect for Vec<T> {
    fn name() -> String {
        "Vec<".to_string() + &T::name() + ">"
    }

    fn def() -> DefType {
        DefType::Vec(DefVec {
            name: Self::name(),
            size: mem::size_of::<Vec<T>>(),
            align: mem::align_of::<Vec<T>>(),
            value_type_name: T::name(),
        })
    }

    fn collect_child_types(types: &mut HashMap<String, DefType>) {
        T::collect_types(types);
    }
}

impl<T: Inspect> Inspect for Option<T> {
    fn name() -> String {
        "Option<".to_string() + &T::name() + ">"
    }

    fn def() -> DefType {
        DefType::Option(DefOption {
            name: Self::name(),
            size: mem::size_of::<Option<T>>(),
            align: mem::align_of::<Option<T>>(),
            value_type_name: T::name(),
        })
    }

    fn collect_child_types(types: &mut HashMap<String, DefType>) {
        T::collect_types(types);
    }
}
