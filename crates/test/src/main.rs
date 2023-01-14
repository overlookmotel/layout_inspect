#![allow(dead_code)]

use struct_inspect::{inspect, types_to_json};
use types::Foo;

pub fn main() {
    let types = inspect::<Foo>();
    dbg!(&types);

    let json = types_to_json(&types);
    println!("{}", &json);
}
