# layout_inspect

Inspect type layout of structs and enums at runtime.

Primary use case is to enable a serializer/deserializer to parse/construct types from raw memory blocks. For this purpose, it would be preferable to get type information at compile time, rather than runtime, but this isn't currently possible.

This crate is in early development and lacks support for some common types at present.

## Installation

Add to `Cargo.toml`:

```toml
layout_inspect = "0.1"
```

## Usage

### `Inspect` trait

Derive `Inspect` for the types you wish to inspect.

```rust
use layout_inspect::Inspect;

#[derive(Inspect)]
struct Foo {
  number: SomeNumber,
  maybe: bool,
}

#[derive(Inspect)]
enum SomeNumber {
  Big(u64),
  Small(u8),
}
```

All types used within a struct/enum which derives `Inspect` must themselves implement `Inspect`.

`Inspect` is already implemented for many common stdlib types e.g. `u*`, `i*`, `bool`, `str`, `String`, `Box`, `Vec`, `Option`, `Rc`, `PhantomData`, `()`.

Tuples, arrays, tuple structs, and references (`&T`) are not supported yet.

### `InspectSize` trait

For types which are not `Sized`, `InspectSize` trait must also be implemented.

There is no derive macro for `InspectSize` at present, so it needs to be manually implemented.

```rust
use layout_inspect::{Inspect, InspectSize};

#[derive(Inspect)]
struct Foo {
  number: u32,
  maybe: [u8],
}

impl InspectSize for Foo {
  fn align() -> Option<usize> {
    Some(std::mem::align_of::<u32>())
  }
}
```

### Inspecting

```rust
use layout_inspect::inspect;
let types = inspect::<Foo>();
```

`inspect()` traverses the graph of all types `Foo` contains, recursively. It returns a `Vec` of `layout_inspect::defs::DefType` objects, comprising type definitions of all reachable types.

```rust
types = [
  DefStruct {
    name: "Foo",
    size: Some(24),
    align: Some(8),
    fields: [
      DefStructField { name: "number", type_id: 1, offset: 0 },
      DefStructField { name: "maybe", type_id: 4, offset: 16 },
    ],
  },
  DefEnum {
    name: "SomeNumber",
    size: 16,
    align: 8,
    variants: [
      DefEnumVariant { name: "Big", discriminant: 0, value_type_id: Some(2) },
      DefEnumVariant { name: "Small", discriminant: 1, value_type_id: Some(3) },
    ],
  },
  DefPrimitive { name: "u64", size: 8, align: 8 },
  DefPrimitive { name: "u8", size: 1, align: 1 },
  DefPrimitive { name: "bool", size: 1, align: 1 },
]
```

`type_id` / `value_type_id` is the index of the `DefType` in `types`.

## Features

### `stable` / `nightly`

`stable` feature is enabled by default.

`nightly` feature enables a small optimization only available on nightly Rust. Disable default features if you use this (`stable` and `nightly` features cannot be enabled simultaneously).

### `derive`

Enable `#[derive(Inspect)]` macro (enabled by default).

### `serde`

Implements [serde](https://serde.rs/)'s `Serialize` and `Deserialize` traits for `DefType`.

```toml
# Cargo.toml
[dependencies]
layout_inspect = { features = [ "serde" ] }
serde_json = "1.0"
```

```rust
let types = inspect::<Foo>();
let json = serde_json::to_string(types).unwrap();
```

```json
[{"kind":"struct","name":"Foo","size":24,"align":8,"fields":[...]},...]
```
