use proc_macro2::TokenStream;
use quote::quote;
use regex::Regex;
use syn::{
	parse_macro_input, AttrStyle, Data, DataEnum, DataStruct, DeriveInput, Expr, Fields, Ident, Lit,
	Meta, NestedMeta,
};

// TODO Support generic structs e.g. `struct Foo<T> { inner: T }`

#[proc_macro_derive(Inspect)]
pub fn inspect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	inspect_impl(input).into()
}

fn inspect_impl(input: DeriveInput) -> TokenStream {
	let type_name = input.ident;

	match input.data {
		Data::Struct(ref data) => derive_struct(data, type_name),
		Data::Enum(ref data) => derive_enum(data, type_name),
		_ => todo!(),
	}
}

fn derive_struct(data: &DataStruct, type_name: Ident) -> TokenStream {
	let (field_defs, field_collect_types): (Vec<_>, Vec<_>) = match data.fields {
		Fields::Named(ref fields) => fields.named.iter().map(|field| {
			let name = field.ident.as_ref().expect("Missing field name");
			let name_str = name.to_string();
			let mut js_name_str = name_str.clone();
			let mut flatten = quote! { false };

			for attr in &field.attrs {
				if attr.style == AttrStyle::Outer && attr.path.is_ident("serde") {
					let meta = attr.parse_meta().unwrap();
					if let Meta::List(list) = meta {
						for item in list.nested {
							if let NestedMeta::Meta(meta) = item {
								match meta {
									Meta::NameValue(name_value) => {
										if name_value.path.is_ident("rename") {
											match &name_value.lit {
												Lit::Str(s) => {
													js_name_str = s.value();
												}
												_ => panic!("Unexpected serde rename tag"),
											}
										}
									}
									Meta::Path(path) => {
										if path.is_ident("flatten") {
											flatten = quote! { true };
										}
									}
									_ => {}
								}
							}
						}
					}
				}
			}

			let ty = &field.ty;
			let field_def = quote! {
					::struct_inspect::defs::DefStructField {
							name: #name_str.to_string(),
							js_name: #js_name_str.to_string(),
							type_name: <#ty as ::struct_inspect::Inspect>::name(),
							offset: ::struct_inspect::__offset_of!(#type_name, #name),
							flatten: #flatten,
					}
			};
			let collect_type = quote! {
					<#ty as ::struct_inspect::Inspect>::collect_types(types);
			};
			(field_def, collect_type)
		}),
		Fields::Unnamed(ref _fields) => todo!(),
		Fields::Unit => todo!(),
	}
	.unzip();

	let type_name_str = type_name.to_string();
	quote! {
			#[automatically_derived]
			impl ::struct_inspect::Inspect for #type_name {
					fn name() -> ::std::string::String {
							#type_name_str.to_string()
					}

					fn def() -> ::struct_inspect::defs::DefType {
							::struct_inspect::defs::DefType::Struct(
									::struct_inspect::defs::DefStruct {
											name: Self::name(),
											size: ::std::mem::size_of::<#type_name>(),
											align: ::std::mem::align_of::<#type_name>(),
											fields: vec![#(#field_defs),*],
									}
							)
					}

					fn collect_child_types(
							types: &mut ::std::collections::HashMap<
									::std::string::String,
									::struct_inspect::defs::DefType
							>
					) {
							#(#field_collect_types)*
					}
			}
	}
}

fn derive_enum(data: &DataEnum, type_name: Ident) -> TokenStream {
	let mut next_discriminant: u64 = 0;

	let (variant_defs, variant_collect_types): (Vec<_>, Vec<_>) = data
		.variants
		.iter()
		.map(|variant| {
			let (value, value_type_name, collect_type) = match &variant.fields {
				Fields::Unit => {
					let doc_comments: Vec<_> = variant
						.attrs
						.iter()
						.filter(|attr| attr.style == AttrStyle::Outer && attr.path.is_ident("doc"))
						.collect();
					assert!(
						doc_comments.len() == 1,
						"{} enum {} option has {} value doc comment",
						type_name,
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
								type_name, variant.ident
							),
						},
						_ => panic!(
							"Unexpected value doc comment for {} enum {} option",
							type_name, variant.ident
						),
					};

					let regex = Regex::new("^ `(.+)`$").unwrap();
					let value = &regex.captures(&value).unwrap()[1];

					let value = quote! { ::std::option::Option::Some(#value.to_string()) };
					let value_type_name = quote! { ::std::option::Option::None };
					let collect_type = quote! {};
					(value, value_type_name, collect_type)
				}
				Fields::Unnamed(ref fields) => {
					let unnamed = &fields.unnamed;
					assert!(unnamed.len() == 1);
					let ty = &unnamed.first().unwrap().ty;
					let value = quote! { ::std::option::Option::None };
					let value_type_name = quote! {
							::std::option::Option::Some(<#ty as ::struct_inspect::Inspect>::name())
					};
					let collect_type = quote! {
							<#ty as ::struct_inspect::Inspect>::collect_types(types);
					};
					(value, value_type_name, collect_type)
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

			let type_name_str = variant.ident.to_string();
			let variant_def = quote! {
					::struct_inspect::defs::DefEnumVariant {
							name: #type_name_str.to_string(),
							discriminant: #discriminant,
							value: #value,
							value_type_name: #value_type_name,
					}
			};

			(variant_def, collect_type)
		})
		.unzip();

	let type_name_str = type_name.to_string();
	quote! {
			#[automatically_derived]
			impl Inspect for #type_name {
					fn name() -> ::std::string::String {
							#type_name_str.to_string()
					}

					fn def() -> ::struct_inspect::defs::DefType {
							::struct_inspect::defs::DefType::Enum(
									::struct_inspect::defs::DefEnum {
											name: Self::name(),
											size: ::std::mem::size_of::<#type_name>(),
											align: ::std::mem::align_of::<#type_name>(),
											variants: ::std::vec![#(#variant_defs),*],
									}
							)
					}

					fn collect_child_types(
							types: &mut ::std::collections::HashMap<
									::std::string::String,
									::struct_inspect::defs::DefType
							>
					) {
							#(#variant_collect_types)*
					}
			}
	}
}
