use serde::{Deserialize, Serialize};
use struct_inspect::Inspect;

#[derive(Inspect, Serialize, Deserialize)]
pub struct Foo {
    pub small: u8,
    pub big: u32,
    #[serde(rename = "middle")]
    pub mid: u16,
    #[serde(flatten)]
    pub bar: Bar,
    pub arr: Vec<Qux>,
    pub switch: Switch,
    pub recurse: Option<Box<Foo>>,
}

#[derive(Inspect, Serialize, Deserialize)]
pub struct Bar {
    pub smaller: u8,
}

#[derive(Inspect, Serialize, Deserialize)]
pub enum Qux {
    Sun(Foo),
    Moon(Bar),
}

#[derive(Inspect, Serialize, Deserialize)]
pub enum Switch {
    /// `on`
    On,
    /// `off`
    Off,
}
