use struct_inspect::Inspect;

#[derive(Inspect)]
pub struct Foo {
    pub small: u8,
    pub big: u32,
    pub mid: u16,
    pub arr: Vec<u8>,
    pub recurse: Option<Box<Foo>>,
}
