#![allow(dead_code)]

use struct_inspect::Inspect;

struct Foo {
    pub bar: u32,
}

impl Inspect for Foo {}

pub fn main() {
    let foo = Foo { bar: 16 };
    foo.say_hello();
}
