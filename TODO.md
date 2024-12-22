# TODO

* See tracking issue on Github for list

* Get offset of `ptr`, `len` and `cap` fields in `Vec`

* Niches + use niches in `Option`

* Use this for offset of Option `Some` variant:

https://github.com/rust-lang/rfcs/pull/3308#issuecomment-1387691456

```rust
const fn get_some_offset() -> isize {
	if mem::size_of::<Option<T>>() == mem::size_of::<T>() {
		// niche optimization means the `T` is always stored at the same position as the Option.
		0
	} else {
		assert!(mem::size_of::<Option<T>>() == mem::size_of::<Option<mem::MaybeUninit<T>>>());
		let some_uninit = Some(mem::MaybeUninit::<T>::uninit());
		// SAFETY: This gets the byte offset of the `Some(_)` value following the fact that
		// niche optimization is not active, and thus Option<T> and Option<MaybeUninit<t>> share
		// the same layout, which we have asserted above.
		unsafe {
			(some_uninit.as_ref().unwrap() as *const mem::MaybeUninit<T>)
				.byte_offset_from(&some_uninit as *const Option<mem::MaybeUninit<T>>)
		}
	}
}
```

* Composite enum fields are not same as a tuple of those fields:

```rust
enum E1 { A, B(u16, u8) }
enum E2 { A, B((u16, u8)) }
assert!(mem::size_of::<E1>() == 4);
assert!(mem::size_of::<E2>() == 6);
```

Therefore, `DefEnumVariant::value_type_id` which is currently `Option<TypeId>` needs to be `Vec<TypeId>`.

Also need to support named subfields:

```rust
enum E3 { A, B { x: u8, y: u16 } }
assert!(mem::size_of::<E3>() == 4);
```

Therefore actually needs to be:

```rust
pub struct DefEnumVariant {
	pub name: String,
	pub discriminant: u64,
	pub ser_value: Option<String>,
	pub value_type_id: Vec<DefEnumVariantField>,
}

pub struct DefEnumVariantField {
	pub name: Option<String>,
	pub type_id: TypeId,
}
```
