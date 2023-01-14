#![feature(log_syntax)]
#![feature(const_trait_impl)]
#![feature(const_refs_to_cell)]
#![feature(const_mut_refs)]
// #![allow(dead_code)]

use std::cell::UnsafeCell;

#[const_trait]
trait Inspect {
    // To be defined in impls
    fn name() -> &'static str;
    fn log();
}

struct Foo {}

impl const Inspect for Foo {
    fn name() -> &'static str {
        "Foo"
    }

    fn log() {
        // log_syntax!(types: "Foo");
        Bar::log();
    }
}

struct Bar {}

impl const Inspect for Bar {
    fn name() -> &'static str {
        "Bar"
    }

    fn log() {
        // log_syntax!(types: "Bar");
        Foo::log();
    }
}

impl<T: Inspect> const Inspect for Vec<T> {
    fn name() -> &'static str {
        "Vec<T>"
    }

    fn log() {
        // log_syntax!(types: "Vec<T>");
        // Bar::log();
    }
}

// const _: () = Foo::log();
// const _: () = Bar::log();
// const _: () = <Vec<Foo> as Inspect>::log();

#[allow(dead_code)]
const VISITED: UnsafeCell<bool> = UnsafeCell::new(false);
#[allow(dead_code)]
const fn mutate_cell() {
    // if unsafe { *VISITED.get() } == true {
    //    return;
    // }
    unsafe { *(VISITED.get() as *mut bool) = true };
}

const _: () = {
    mutate_cell();
};

#[allow(dead_code)]
const fn test() {
    #[warn(dead_code)]
    const WARNING: bool = true;
}

const _: () = {
    // test();
};

pub fn main() {
    println!("{:?}", unsafe { *VISITED.get() });
}
