pub use std::mem;

use crate::{
    defs::{DefPrimitive, DefType},
    Inspect,
};

macro_rules! primitive {
    ($type:ty) => {
        impl Inspect for $type {
            fn name() -> String {
                // Uppercase type name
                let mut chars: Vec<char> = stringify!($type).chars().collect();
                chars[0] = chars[0].to_uppercase().nth(0).unwrap();
                chars.into_iter().collect()
            }

            fn def() -> DefType {
                DefType::Primitive(DefPrimitive {
                    name: Self::name(),
                    size: mem::size_of::<$type>(),
                    align: mem::align_of::<$type>(),
                })
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
