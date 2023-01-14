#![allow(dead_code)]

pub struct Foo {
    bar: Bar,
}
type FooFoo = Foo;

pub struct Bar {
    small: u8,
    medium: u16,
    big: u32,
    // vec: Option<Vec<u8>>,
}
type BarBar = Bar;

fn main() {
    let _: Option<Foo> = None;
}
