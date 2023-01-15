use std::mem::{align_of, size_of};

use struct_inspect::{
    defs::{DefOption, DefType},
    Inspect,
};

#[test]
fn option_primitive() {
    assert_eq!(Option::<u8>::name(), "Option<U8>");
    assert_eq!(
        Option::<u8>::def(),
        DefType::Option(DefOption {
            name: "Option<U8>".to_string(),
            size: size_of::<u8>() * 2,
            align: align_of::<u8>(),
            value_type_name: "U8".to_string(),
        })
    );
}

#[test]
fn option_primitive_with_niche() {
    assert_eq!(Option::<bool>::name(), "Option<Bool>");
    assert_eq!(
        Option::<bool>::def(),
        DefType::Option(DefOption {
            name: "Option<Bool>".to_string(),
            size: size_of::<bool>(),
            align: align_of::<bool>(),
            value_type_name: "Bool".to_string(),
        })
    );
}

#[test]
fn option_struct() {
    #[derive(Inspect)]
    struct Foo {
        small: u8,
        big: u32,
    }

    assert_eq!(Option::<Foo>::name(), "Option<Foo>");
    assert_eq!(
        Option::<Foo>::def(),
        DefType::Option(DefOption {
            name: "Option<Foo>".to_string(),
            size: size_of::<Foo>() + align_of::<Foo>(),
            align: align_of::<Foo>(),
            value_type_name: "Foo".to_string(),
        })
    );
}

#[test]
fn option_struct_with_niche() {
    use std::num::NonZeroU32;

    #[derive(Inspect)]
    struct Foo {
        small: u8,
        big: NonZeroU32,
    }

    assert_eq!(Option::<Foo>::name(), "Option<Foo>");
    assert_eq!(
        Option::<Foo>::def(),
        DefType::Option(DefOption {
            name: "Option<Foo>".to_string(),
            size: size_of::<Foo>(),
            align: align_of::<Foo>(),
            value_type_name: "Foo".to_string(),
        })
    );
}

#[test]
fn option_box() {
    assert_eq!(Option::<Box<u8>>::name(), "Option<Box<U8>>");
    assert_eq!(
        Option::<Box<u8>>::def(),
        DefType::Option(DefOption {
            name: "Option<Box<U8>>".to_string(),
            size: size_of::<usize>(),
            align: align_of::<usize>(),
            value_type_name: "Box<U8>".to_string(),
        })
    );
}

#[test]
fn option_vec() {
    assert_eq!(Option::<Vec<u8>>::name(), "Option<Vec<U8>>");
    assert_eq!(
        Option::<Vec<u8>>::def(),
        DefType::Option(DefOption {
            name: "Option<Vec<U8>>".to_string(),
            size: size_of::<usize>() * 3,
            align: align_of::<usize>(),
            value_type_name: "Vec<U8>".to_string(),
        })
    );
}
