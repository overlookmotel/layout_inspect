use std::{collections::hash_map::HashMap, mem};

pub use crate::Inspect;

impl<T: Inspect> Inspect for Box<T> {
    fn name() -> String {
        "Box".to_string() + &T::name()
    }

    fn kind() -> String {
        "box".to_string()
    }

    fn size() -> usize {
        mem::size_of::<Box<T>>()
    }

    fn align() -> usize {
        mem::align_of::<Box<T>>()
    }

    fn json() -> Option<String> {
        Some(format!("\"childType\":\"{}\"", &T::name()))
    }

    fn collect_child_types(types: &mut HashMap<String, String>) {
        T::collect_types(types);
    }
}

impl<T: Inspect> Inspect for Vec<T> {
    fn name() -> String {
        "Vec".to_string() + &T::name()
    }

    fn kind() -> String {
        "vec".to_string()
    }

    fn size() -> usize {
        mem::size_of::<Vec<T>>()
    }

    fn align() -> usize {
        mem::align_of::<Vec<T>>()
    }

    fn json() -> Option<String> {
        Some(format!("\"childType\":\"{}\"", &T::name()))
    }

    fn collect_child_types(types: &mut HashMap<String, String>) {
        T::collect_types(types);
    }
}

impl<T: Inspect> Inspect for Option<T> {
    fn name() -> String {
        "Option".to_string() + &T::name()
    }

    fn kind() -> String {
        "option".to_string()
    }

    fn size() -> usize {
        mem::size_of::<Option<T>>()
    }

    fn align() -> usize {
        mem::align_of::<Option<T>>()
    }

    fn json() -> Option<String> {
        // TODO Is this always correct?
        // Could maybe create a dummy `<Option<T>>` with `MaybeUninit` and then get the
        // memory addresses of the `Option<T>` and the `T` it encloses. The
        // difference between the two is the offset.
        let value_offset = mem::size_of::<Option<T>>() - mem::size_of::<T>();

        // Get value that represents `None` if a niche is used
        let none_value: String = if value_offset == 0 {
            // Not sure how to deal with this if type cannot cleanly transmute to a Uint,
            // so bail out with panic in such cases.
            // At least this is enough to handle `Option<T>` where `T` is a fieldless
            // enum with less than 256 options, a `Box`, or a `Vec`.
            let size = mem::size_of::<T>();
            let align = mem::align_of::<T>();
            let none_value: u128 = if size == 1 {
                assert_eq!(size, mem::size_of::<u8>());
                assert_eq!(align, mem::align_of::<u8>());
                let u8_ref: &u8 = unsafe { mem::transmute(&(None as Option<T>)) };
                *u8_ref as u128
            } else if size == 2 {
                assert_eq!(size, mem::size_of::<u16>());
                assert_eq!(align, mem::align_of::<u16>());
                let u16_ref: &u16 = unsafe { mem::transmute(&(None as Option<T>)) };
                *u16_ref as u128
            } else if size == 4 {
                assert_eq!(size, mem::size_of::<u32>());
                assert_eq!(align, mem::align_of::<u32>());
                let u32_ref: &u32 = unsafe { mem::transmute(&(None as Option<T>)) };
                *u32_ref as u128
            } else if size == 8 {
                assert_eq!(size, mem::size_of::<u64>());
                assert_eq!(align, mem::align_of::<u64>());
                let u64_ref: &u64 = unsafe { mem::transmute(&(None as Option<T>)) };
                *u64_ref as u128
            } else if size == 16 {
                assert_eq!(size, mem::size_of::<u128>());
                assert_eq!(align, mem::align_of::<u128>());
                let u128_ref: &u128 = unsafe { mem::transmute(&(None as Option<T>)) };
                *u128_ref
            } else {
                panic!(
                    "Unable to determine None niche value for {} with size {} and align {}",
                    &Self::name(),
                    size,
                    align
                );
            };
            format!("{}", none_value)
        } else {
            "null".to_string()
        };

        Some(format!(
            "\"childType\":\"{}\",\"valueOffset\":{},\"noneValue\":{}",
            &T::name(),
            value_offset,
            none_value
        ))
    }

    fn collect_child_types(types: &mut HashMap<String, String>) {
        T::collect_types(types);
    }
}
