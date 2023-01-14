use struct_inspect::{inspect, types_to_json};
use types::Foo;

pub fn main() {
    println!("cargo:warning=running build script");

    let types = inspect::<Foo>();
    let json = types_to_json(&types);
    println!("cargo:warning=types:{}", &json);
}
