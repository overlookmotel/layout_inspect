use proc_macro2;
use syn::{parse_macro_input, Data, DeriveInput};

mod structs;
use structs::derive_struct;
mod enums;
use enums::derive_enum;

#[proc_macro_derive(Inspect)]
pub fn inspect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	inspect_impl(input).into()
}

fn inspect_impl(input: DeriveInput) -> proc_macro2::TokenStream {
	match input.data {
		Data::Struct(ref data) => derive_struct(data, input.ident),
		Data::Enum(ref data) => derive_enum(data, input.ident),
		Data::Union(ref _data) => todo!("Deriving `Inspect` on Unions not supported"),
	}
}
