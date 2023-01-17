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
						type_ident,
						variant.ident,
						match doc_comments.len() {
							0 => "no",
							_ => "more than one",
						},
					);

					let meta = doc_comments[0].parse_meta().unwrap();
					let ser_value = match meta {
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
					let ser_value = &regex.captures(&ser_value).unwrap()[1];

					let ser_value = quote! { ::std::option::Option::Some(#ser_value.to_string()) };
					let value_type_id = quote! { ::std::option::Option::None };
					(ser_value, value_type_id)
				}
				Fields::Unnamed(ref fields) => {
					let unnamed = &fields.unnamed;
					assert!(unnamed.len() == 1);
					let ty = &unnamed.first().unwrap().ty;
					let ser_value = quote! { ::std::option::Option::None };
					let value_type_id = quote! {
							::std::option::Option::Some(collector.collect::<#ty>())
					};
					(ser_value, value_type_id)
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

			let variant_ident = &variant.ident;
			quote! {
					::struct_inspect::defs::DefEnumVariant {
							name: stringify!(#variant_ident).to_string(),
							discriminant: #discriminant,
							ser_value: #ser_value,
							value_type_id: #value_type_id,
					}
			}
		})
		.collect();

	quote! {
			#[automatically_derived]
			impl Inspect for #type_ident {
					fn name() -> ::std::string::String {
							stringify!(#type_ident).to_string()
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
