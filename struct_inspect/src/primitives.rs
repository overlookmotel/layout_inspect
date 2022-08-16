pub use std::mem;

use crate::{Inspect, TypeDef, TypeKind};

macro_rules! primitive {
    ($type:ty) => {
        impl Inspect for $type {
            fn type_def() -> TypeDef {
                TypeDef {
                    name: stringify!($type),
                    kind: TypeKind::Primitive,
                    len: mem::size_of::<$type>(),
                    child: None,
                }
            }
        }
    };
}

primitive!(bool);
primitive!(u8);
primitive!(u16);
primitive!(u32);
primitive!(u64);
primitive!(u128);
primitive!(i8);
primitive!(i16);
primitive!(i32);
primitive!(i64);
primitive!(i128);
primitive!(f32);
primitive!(f64);
primitive!(usize);
primitive!(isize);
