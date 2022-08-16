pub use std::mem;

use crate::{Inspect, TypeDef, TypeKind};

macro_rules! primitive {
    ($type:ty) => {
        impl Inspect for $type {
            fn name() -> String {
                // Uppercase type name
                let mut chars: Vec<char> = stringify!($type).chars().collect();
                chars[0] = chars[0].to_uppercase().nth(0).unwrap();
                chars.into_iter().collect()
            }

            fn size() -> usize {
                mem::size_of::<$type>()
            }

            fn align() -> usize {
                mem::align_of::<$type>()
            }

            fn type_def() -> TypeDef {
                TypeDef {
                    name: {
                        // Uppercase type name
                        let mut chars: Vec<char> = stringify!($type).chars().collect();
                        chars[0] = chars[0].to_uppercase().nth(0).unwrap();
                        chars.into_iter().collect()
                    },
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
