use proc_macro2::TokenStream;
use quote::quote;
use syn::{
	parse_quote, AttrStyle, DataEnum, Expr, Fields, FieldsUnnamed, GenericParam, Generics, Ident,
	Lit, Meta, NestedMeta,
};

// TODO: Support generic enums e.g. `enum Maybe<T> { Some(T), Nothing }`
// TODO: Support `#[serde(rename_all = "camelCase")]` (and other cases)
// https://serde.rs/container-attrs.html#rename_all
// TODO: Should `discriminant` be `i64` not `u64`?
// TODO: For fieldless enums, use e.g. `Foo::Bar as u64` to get discriminants.
// Discrimants can be defined as a const expression which we can't parse
// e.g. `enum X { Y = mem::size_of::<u32>() }`
// NB Only legal for enums where all variants are fieldless.
// That includes e.g. `enum X { Tuple(), Struct{} }`.
// Also see: https://rust-lang.github.io/rfcs/2363-arbitrary-enum-discriminant.html
// Maybe possible to obtain discriminant values for field-ful enums
// if enum is `#[repr(u../i..)]`.

pub fn derive_enum(data: DataEnum, ident: Ident, mut generics: Generics) -> TokenStream {
	let mut next_discriminant: u64 = 0;

	let variant_defs: Vec<_> = data
		.variants
		.into_iter()
		.map(|variant| {
			let (ser_value, value_type_id) = match variant.fields {
				Fields::Unit => {
					let mut ser_value = variant.ident.to_string();

					// Find `#[serde(rename)]` attribute
					for attr in &variant.attrs {
						if attr.style == AttrStyle::Outer && attr.path.is_ident("serde") {
							let meta = attr.parse_meta().unwrap();
							if let Meta::List(list) = meta {
								for item in list.nested {
									if let NestedMeta::Meta(Meta::NameValue(name_value)) = item {
										if name_value.path.is_ident("rename") {
											if let Lit::Str(s) = name_value.lit {
												ser_value = s.value();
											} else {
												panic!("Unexpected serde rename tag");
											}
										}
									}
								}
							}
						}
					}

					let ser_value = quote! { Some(#ser_value.to_string()) };
					let value_type_id = quote! { None };
					(ser_value, value_type_id)
				}
				Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
					assert!(unnamed.len() == 1);
					let ty = &unnamed.first().unwrap().ty;
					let ser_value = quote! { None };
					let value_type_id = quote! { Some(collector.collect::<#ty>()) };
					(ser_value, value_type_id)
				}
				Fields::Named(_) => todo!(),
			};

			let discriminant = match variant.discriminant {
				Some(discriminant) => {
					match discriminant.1 {
						Expr::Lit(expr_lit) => {
							match expr_lit.lit {
								Lit::Int(int) => int.base10_parse::<u64>().unwrap(),
								_ => todo!(),
							}
						}
						_ => todo!(),
					}
				}
				None => next_discriminant,
			};
			next_discriminant = discriminant + 1;

			let variant_ident = variant.ident;
			quote! {
				DefEnumVariant {
					name: stringify!(#variant_ident).to_string(),
					discriminant: #discriminant,
					ser_value: #ser_value,
					value_type_id: #value_type_id,
				}
			}
		})
		.collect();

	// Add bound `Inspect` to type params
	for param in &mut generics.params {
		if let GenericParam::Type(ref mut type_param) = *param {
			type_param.bounds.push(parse_quote!(Inspect));
		}
	}

	// Return `impl` code
	let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

	quote! {
		const _: () = {
			use ::std::{
				mem,
				option::Option::{self, None, Some},
				string::String,
				stringify, vec,
			};
			use ::layout_inspect::{
				defs::{DefEnum, DefEnumVariant, DefType},
				Inspect, TypesCollector,
			};

			#[automatically_derived]
			impl #impl_generics Inspect for #ident #type_generics #where_clause {
				fn name() -> String {
					stringify!(#ident).to_string()
				}

				fn size() -> Option<usize> {
					Some(mem::size_of::<Self>())
				}

				fn align() -> Option<usize> {
					Some(mem::align_of::<Self>())
				}

				fn def(collector: &mut TypesCollector) -> DefType {
					DefType::Enum(DefEnum {
						name: <Self as Inspect>::name(),
						size: <Self as Inspect>::size().unwrap(),
						align: <Self as Inspect>::align().unwrap(),
						variants: vec![#(#variant_defs),*],
					})
				}
			}
		};
	}
}
