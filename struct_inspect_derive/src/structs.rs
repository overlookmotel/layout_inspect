use proc_macro2::TokenStream;
use quote::quote;
use syn::{AttrStyle, DataStruct, Fields, Ident, Lit, Meta, NestedMeta};

// TODO Support generic structs e.g. `struct Foo<T> { inner: T }`

pub fn derive_struct(data: &DataStruct, type_ident: Ident) -> TokenStream {
	let field_defs: Vec<TokenStream> = match data.fields {
		Fields::Named(ref fields) => fields
			.named
			.iter()
			.map(|field| {
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
				quote! {
						::struct_inspect::defs::DefStructField {
								name: #name_str.to_string(),
								js_name: #js_name_str.to_string(),
								type_id: collector.collect::<#ty>(),
								offset: ::struct_inspect::__offset_of!(#type_ident, #name),
								flatten: #flatten,
						}
				}
			})
			.collect(),
		Fields::Unnamed(ref _fields) => todo!(),
		Fields::Unit => todo!(),
	};

	let type_ident_str = type_ident.to_string();
	quote! {
			#[automatically_derived]
			impl ::struct_inspect::Inspect for #type_ident {
					fn name() -> ::std::string::String {
							#type_ident_str.to_string()
					}

					fn def(collector: &mut ::struct_inspect::TypesCollector) -> ::struct_inspect::defs::DefType {
							::struct_inspect::defs::DefType::Struct(
									::struct_inspect::defs::DefStruct {
											name: Self::name(),
											size: ::std::mem::size_of::<#type_ident>(),
											align: ::std::mem::align_of::<#type_ident>(),
											fields: vec![#(#field_defs),*],
									}
							)
					}
			}
	}
}
