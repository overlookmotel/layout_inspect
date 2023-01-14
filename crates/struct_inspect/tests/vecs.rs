use std::mem::{align_of, size_of};

use struct_inspect::{
    defs::{DefType, DefVec},
    Inspect,
};

#[test]
fn vec_primitive() {
    assert_eq!(Vec::<u8>::name(), "Vec<U8>");
    assert_eq!(
        Vec::<u8>::def(),
        DefType::Vec(DefVec {
            name: "Vec<U8>".to_string(),
            size: size_of::<usize>() * 3,
            align: align_of::<usize>(),
            value_type_name: "U8".to_string(),
        })
    );
}

#[test]
fn vec_struct() {
    #[derive(Inspect)]
    struct Foo {
        small: u8,
        big: u128,
    }

    assert_eq!(Vec::<Foo>::name(), "Vec<Foo>");
    assert_eq!(
        Vec::<Foo>::def(),
        DefType::Vec(DefVec {
            name: "Vec<Foo>".to_string(),
            size: size_of::<usize>() * 3,
            align: align_of::<usize>(),
            value_type_name: "Foo".to_string(),
        })
    );
}
