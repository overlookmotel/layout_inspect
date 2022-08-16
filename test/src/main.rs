#![allow(dead_code)]

use struct_inspect::Inspect;

#[derive(Inspect)]
pub struct Foo {
    pub small: u8,
    pub big: u32,
    pub mid: u16,
    pub arr: Vec<u8>,
    pub recurse: Option<Box<Foo>>,
}

impl Default for Foo {
    fn default() -> Self {
        Foo {
            small: 10,
            big: 1 * 256 + 2,
            mid: 3 * 256 + 4,
            arr: vec![1, 2, 3],
            recurse: None,
        }
    }
}

pub fn main() {
    // let foo = Foo::default();
    println!("Foo type: {:?}", Foo::type_def());
    println!("Foo fields:");
    for field in Foo::fields_def().unwrap() {
        println!("{:?}", field);
    }
}
