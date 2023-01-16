use proc_macro2::TokenStream;
use quote::quote;
use syn::{AttrStyle, DataStruct, Field, Fields, FieldsNamed, Ident, Lit, Meta, NestedMeta};

// TODO Support generic structs e.g. `struct Foo<T> { inner: T }`
// TODO Support `#[serde(rename_all = "camelCase")]` (and other variants)
// https://serde.rs/container-attrs.html#rename_all

pub fn derive_struct(data: &DataStruct, type_ident: Ident) -> TokenStream {
	let field_defs: Vec<TokenStream> = match data.fields {
		Fields::Named(ref fields) => get_named_field_defs(fields),
		Fields::Unnamed(ref _fields) => todo!("Unnamed struct fields not supported"),
		Fields::Unit => todo!("Unit struct fields not supported"),
	};

	quote! {
			#[automatically_derived]
			impl ::struct_inspect::Inspect for #type_ident {
					fn name() -> ::std::string::String {
							stringify!(#type_ident).to_string()
					}

					fn def(collector: &mut ::struct_inspect::TypesCollector) -> ::struct_inspect::defs::DefType {
							::struct_inspect::defs::DefType::Struct(
									::struct_inspect::defs::DefStruct {
											name: Self::name(),
											size: ::std::mem::size_of::<Self>(),
											align: ::std::mem::align_of::<Self>(),
											fields: vec![#(#field_defs),*],
									}
							)
					}
			}
	}
}

fn get_named_field_defs(fields: &FieldsNamed) -> Vec<TokenStream> {
	fields
		.named
		.iter()
		.map(|field| get_named_field_def(field))
		.collect()
}

fn get_named_field_def(field: &Field) -> TokenStream {
	// Get field name
	let name = field.ident.as_ref().expect("Missing field name");

	// Search field attributes for `#[serde(rename = "x")]` or `#[serde(flatten)]`
	let mut js_name: Option<String> = None;
	let mut flatten = false;

	for attr in &field.attrs {
		if attr.style == AttrStyle::Outer && attr.path.is_ident("serde") {
			let meta = attr.parse_meta().unwrap();
			if let Meta::List(list) = meta {
				for item in list.nested {
					if let NestedMeta::Meta(meta) = item {
						match meta {
							// `#[serde(rename = "x")]`
							Meta::NameValue(name_value) => {
								if name_value.path.is_ident("rename") {
									match &name_value.lit {
										Lit::Str(s) => {
											js_name = Some(s.value());
										}
										_ => panic!("Unexpected serde rename tag"),
									}
								}
							}
							// `#[serde(flatten)]`
							Meta::Path(path) => {
								if path.is_ident("flatten") {
									flatten = true;
								}
							}
							_ => {}
						}
					}
				}
			}
		}
	}

	let js_name = js_name.unwrap_or_else(|| name.to_string());

	// Return field def
	let ty = &field.ty;
	quote! {
			::struct_inspect::defs::DefStructField {
					name: stringify!(#name).to_string(),
					js_name: #js_name.to_string(),
					type_id: collector.collect::<#ty>(),
					offset: ::struct_inspect::__offset_of!(Self, #name),
					flatten: #flatten,
			}
	}
}
