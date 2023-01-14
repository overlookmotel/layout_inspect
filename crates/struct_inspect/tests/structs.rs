use std::mem::{align_of, size_of};

use struct_inspect::{
    defs::{DefStruct, DefStructField, DefType},
    Inspect,
};

// TODO Test for tuple struct - not implemented yet

#[test]
fn struct_single_field() {
    #[derive(Inspect)]
    struct Foo {
        num: u8,
    }

    assert_eq!(Foo::name(), "Foo");
    assert_eq!(
        Foo::def(),
        DefType::Struct(DefStruct {
            name: "Foo".to_string(),
            size: size_of::<Foo>(),
            align: align_of::<Foo>(),
            fields: vec![DefStructField {
                name: "num".to_string(),
                js_name: "num".to_string(),
                type_name: "U8".to_string(),
                offset: 0,
                flatten: false,
            }]
        })
    );
}

#[test]
fn struct_empty() {
    #[derive(Inspect)]
    struct Foo {}

    assert_eq!(Foo::name(), "Foo");
    assert_eq!(
        Foo::def(),
        DefType::Struct(DefStruct {
            name: "Foo".to_string(),
            size: 0,
            align: 1,
            fields: vec![]
        })
    );
}

#[test]
fn struct_multiple_fields() {
    #[derive(Inspect)]
    struct Foo {
        small: u8,
        medium: u16,
        veccy: Vec<u8>,
        recurse: Option<Box<Foo>>,
    }

    assert_eq!(Foo::name(), "Foo");
    assert_eq!(
        Foo::def(),
        DefType::Struct(DefStruct {
            name: "Foo".to_string(),
            size: size_of::<Foo>(),
            align: align_of::<Foo>(),
            fields: vec![
                DefStructField {
                    name: "small".to_string(),
                    js_name: "small".to_string(),
                    type_name: "U8".to_string(),
                    offset: size_of::<usize>() * 4 + size_of::<u16>(),
                    flatten: false
                },
                DefStructField {
                    name: "medium".to_string(),
                    js_name: "medium".to_string(),
                    type_name: "U16".to_string(),
                    offset: size_of::<usize>() * 4,
                    flatten: false
                },
                DefStructField {
                    name: "veccy".to_string(),
                    js_name: "veccy".to_string(),
                    type_name: "Vec<U8>".to_string(),
                    offset: 0,
                    flatten: false
                },
                DefStructField {
                    name: "recurse".to_string(),
                    js_name: "recurse".to_string(),
                    type_name: "Option<Box<Foo>>".to_string(),
                    offset: size_of::<usize>() * 3,
                    flatten: false
                }
            ]
        })
    );
}

#[test]
fn struct_with_serde_field_rename() {
    use serde::{Deserialize, Serialize};

    #[derive(Inspect, Deserialize, Serialize)]
    struct Foo {
        #[serde(rename = "bar")]
        num: u8,
    }

    assert_eq!(Foo::name(), "Foo");
    assert_eq!(
        Foo::def(),
        DefType::Struct(DefStruct {
            name: "Foo".to_string(),
            size: size_of::<Foo>(),
            align: align_of::<Foo>(),
            fields: vec![DefStructField {
                name: "num".to_string(),
                js_name: "bar".to_string(),
                type_name: "U8".to_string(),
                offset: 0,
                flatten: false,
            }]
        })
    );
}

#[test]
fn struct_with_serde_field_flatten() {
    use serde::{Deserialize, Serialize};

    #[derive(Inspect, Deserialize, Serialize)]
    struct Foo {
        #[serde(flatten)]
        bar: Bar,
    }

    #[derive(Inspect, Deserialize, Serialize)]
    struct Bar {
        num: u8,
    }

    assert_eq!(Foo::name(), "Foo");
    assert_eq!(
        Foo::def(),
        DefType::Struct(DefStruct {
            name: "Foo".to_string(),
            size: size_of::<Foo>(),
            align: align_of::<Foo>(),
            fields: vec![DefStructField {
                name: "bar".to_string(),
                js_name: "bar".to_string(),
                type_name: "Bar".to_string(),
                offset: 0,
                flatten: true,
            }]
        })
    );
}

#[test]
fn struct_with_serde_field_rename_and_flatten() {
    use serde::{Deserialize, Serialize};

    #[derive(Inspect, Deserialize, Serialize)]
    struct Foo {
        #[serde(flatten, rename = "qux")]
        bar: Bar,
    }

    #[derive(Inspect, Deserialize, Serialize)]
    struct Bar {
        num: u8,
    }

    assert_eq!(Foo::name(), "Foo");
    assert_eq!(
        Foo::def(),
        DefType::Struct(DefStruct {
            name: "Foo".to_string(),
            size: size_of::<Foo>(),
            align: align_of::<Foo>(),
            fields: vec![DefStructField {
                name: "bar".to_string(),
                js_name: "qux".to_string(),
                type_name: "Bar".to_string(),
                offset: 0,
                flatten: true,
            }]
        })
    );
}

#[test]
fn struct_with_serde_field_default() {
    use serde::{Deserialize, Serialize};

    #[derive(Inspect, Deserialize, Serialize)]
    struct Foo {
        #[serde(default)]
        num: u8,
    }

    assert_eq!(Foo::name(), "Foo");
    assert_eq!(
        Foo::def(),
        DefType::Struct(DefStruct {
            name: "Foo".to_string(),
            size: size_of::<Foo>(),
            align: align_of::<Foo>(),
            fields: vec![DefStructField {
                name: "num".to_string(),
                js_name: "num".to_string(),
                type_name: "U8".to_string(),
                offset: 0,
                flatten: false,
            }]
        })
    );
}
