use std::mem;

pub use crate::{Inspect, TypeDef, TypeKind};

impl<T: Inspect> Inspect for Box<T> {
    fn name() -> String {
        "Box".to_string() + &T::name()
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
