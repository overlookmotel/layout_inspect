use proc_macro2::TokenStream;
use quote::quote;
use regex::Regex;
use syn::{AttrStyle, DataEnum, Expr, Fields, Ident, Lit, Meta};

// TODO: Support generic enums e.g. `enum Maybe<T> { Some(T), Nothing }`
// TODO: Support fieldless enums with no value annotation - use discriminant
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

pub fn derive_enum(data: &DataEnum, ident: Ident) -> TokenStream {
	let mut next_discriminant: u64 = 0;

	let variant_defs: Vec<_> = data
		.variants
		.iter()
		.map(|variant| {
			let (ser_value, value_type_id) = match &variant.fields {
				Fields::Unit => {
					let doc_comments: Vec<_> = variant
						.attrs
						.iter()
						.filter(|attr| attr.style == AttrStyle::Outer && attr.path.is_ident("doc"))
						.collect();
					assert!(
						doc_comments.len() == 1,
						"{} enum {} option has {} value doc comment",
						ident,
						variant.ident,
						match doc_comments.len() {
							0 => "no",
							_ => "more than one",
						},
					);

					let meta = doc_comments[0].parse_meta().unwrap();
					let ser_value = match meta {
						Meta::NameValue(name_value) => {
							match &name_value.lit {
								Lit::Str(s) => s.value(),
								_ => {
									panic!(
										"Unexpected value doc comment for {} enum {} option",
										ident, variant.ident
									)
								}
							}
						}
						_ => {
							panic!(
								"Unexpected value doc comment for {} enum {} option",
								ident, variant.ident
							)
						}
					};

					let regex = Regex::new("^ `(.+)`$").unwrap();
					let ser_value = &regex.captures(&ser_value).unwrap()[1];

					let ser_value = quote! { Some(#ser_value.to_string()) };
					let value_type_id = quote! { None };
					(ser_value, value_type_id)
				}
				Fields::Unnamed(ref fields) => {
					let unnamed = &fields.unnamed;
					assert!(unnamed.len() == 1);
					let ty = &unnamed.first().unwrap().ty;
					let ser_value = quote! { None };
					let value_type_id = quote! { Some(collector.collect::<#ty>()) };
					(ser_value, value_type_id)
				}
				Fields::Named(_) => todo!(),
			};

			let discriminant = match &variant.discriminant {
				Some(discriminant) => {
					match &discriminant.1 {
						Expr::Lit(expr_lit) => {
							match &expr_lit.lit {
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

			let variant_ident = &variant.ident;
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
			impl Inspect for #ident {
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
						name: Self::name(),
						size: Self::size().unwrap(),
						align: Self::align().unwrap(),
						variants: vec![#(#variant_defs),*],
					})
				}
			}
		};
	}
}
