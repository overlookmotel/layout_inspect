use proc_macro2::TokenStream;
use quote::quote;
use regex::Regex;
use syn::{AttrStyle, DataEnum, Expr, Fields, Ident, Lit, Meta};

// TODO Support generic enums e.g. `enum Maybe<T> { Some(T), Nothing }`
// TODO Support fieldless enums with no value annotation - use discriminant
// TODO Support `#[serde(rename_all = "camelCase")]` (and other cases)
// https://serde.rs/container-attrs.html#rename_all

pub fn derive_enum(data: &DataEnum, type_ident: Ident) -> TokenStream {
	let mut next_discriminant: u64 = 0;

	let variant_defs: Vec<_> = data
		.variants
		.iter()
		.map(|variant| {
			let (value, value_type_id) = match &variant.fields {
				Fields::Unit => {
					let doc_comments: Vec<_> = variant
						.attrs
						.iter()
						.filter(|attr| attr.style == AttrStyle::Outer && attr.path.is_ident("doc"))
						.collect();
					assert!(
						doc_comments.len() == 1,
						"{} enum {} option has {} value doc comment",
						type_ident,
						variant.ident,
						match doc_comments.len() {
							0 => "no",
							_ => "more than one",
						},
					);

					let meta = doc_comments[0].parse_meta().unwrap();
					let value = match meta {
						Meta::NameValue(name_value) => match &name_value.lit {
							Lit::Str(s) => s.value(),
							_ => panic!(
								"Unexpected value doc comment for {} enum {} option",
								type_ident, variant.ident
							),
						},
						_ => panic!(
							"Unexpected value doc comment for {} enum {} option",
							type_ident, variant.ident
						),
					};

					let regex = Regex::new("^ `(.+)`$").unwrap();
					let value = &regex.captures(&value).unwrap()[1];

					let value = quote! { ::std::option::Option::Some(#value.to_string()) };
					let value_type_id = quote! { ::std::option::Option::None };
					(value, value_type_id)
				}
				Fields::Unnamed(ref fields) => {
					let unnamed = &fields.unnamed;
					assert!(unnamed.len() == 1);
					let ty = &unnamed.first().unwrap().ty;
					let value = quote! { ::std::option::Option::None };
					let value_type_id = quote! {
							::std::option::Option::Some(collector.collect::<#ty>())
					};
					(value, value_type_id)
				}
				Fields::Named(_) => todo!(),
			};

			let discriminant = match &variant.discriminant {
				Some(discriminant) => match &discriminant.1 {
					Expr::Lit(expr_lit) => match &expr_lit.lit {
						Lit::Int(int) => int.base10_parse::<u64>().unwrap(),
						_ => todo!(),
					},
					_ => todo!(),
				},
				None => next_discriminant,
			};
			next_discriminant = discriminant + 1;

			let type_ident_str = variant.ident.to_string();
			let variant_def = quote! {
					::struct_inspect::defs::DefEnumVariant {
							name: #type_ident_str.to_string(),
							discriminant: #discriminant,
							value: #value,
							value_type_id: #value_type_id,
					}
			};

			variant_def
		})
		.collect();

	let type_ident_str = type_ident.to_string();
	quote! {
			#[automatically_derived]
			impl Inspect for #type_ident {
					fn name() -> ::std::string::String {
							#type_ident_str.to_string()
					}

					fn def(collector: &mut ::struct_inspect::TypesCollector) -> ::struct_inspect::defs::DefType {
							::struct_inspect::defs::DefType::Enum(
									::struct_inspect::defs::DefEnum {
											name: Self::name(),
											size: ::std::mem::size_of::<Self>(),
											align: ::std::mem::align_of::<Self>(),
											variants: ::std::vec![#(#variant_defs),*],
									}
							)
					}
			}
	}
}
