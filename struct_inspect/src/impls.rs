pub use std::mem;

pub use crate::{Inspect, TypeDef, TypeKind};

impl<T: Inspect> Inspect for Box<T> {
    fn type_def() -> TypeDef {
        let child_def = T::type_def();
        TypeDef {
            name: "Box",
            kind: TypeKind::Box,
            len: mem::size_of::<Self>(),
            child: Some(Box::new(child_def)),
        }
    }
}

impl<T: Inspect> Inspect for Vec<T> {
    fn type_def() -> TypeDef {
        TypeDef {
            name: "Vec",
            kind: TypeKind::Vec,
            len: mem::size_of::<Self>(),
            child: Some(Box::new(T::type_def())),
        }
    }
}

impl<T: Inspect> Inspect for Option<T> {
    fn type_def() -> TypeDef {
        TypeDef {
            name: "Option",
            kind: TypeKind::Option,
            len: mem::size_of::<Self>(),
            child: Some(Box::new(T::type_def())),
        }
    }
}
