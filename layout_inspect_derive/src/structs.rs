use proc_macro2::TokenStream;
use quote::quote;
use syn::{
	parse_quote, AttrStyle, DataStruct, Field, Fields, FieldsNamed, GenericParam, Generics, Ident,
	Lit, Meta, NestedMeta,
};

// TODO: Support `#[serde(rename_all = "camelCase")]` (and other variants)
// https://serde.rs/container-attrs.html#rename_all

pub fn derive_struct(data: DataStruct, ident: Ident, mut generics: Generics) -> TokenStream {
	// Get field definitions
	let field_defs: Vec<TokenStream> = match data.fields {
		Fields::Named(fields) => get_named_field_defs(fields),
		Fields::Unnamed(_fields) => todo!("Unnamed struct fields not supported"),
		Fields::Unit => todo!("Unit struct fields not supported"),
	};

	// Add bound `Inspect` to type params
	for param in &mut generics.params {
		if let GenericParam::Type(ref mut type_param) = *param {
			type_param.bounds.push(parse_quote!(Inspect));
		}
	}

	// Create code for name
	let sub_types: Vec<TokenStream> = generics
		.params
		.iter()
		.filter_map(|param| {
			match param {
				GenericParam::Type(param) => {
					let ident = &param.ident;
					Some(quote! {&#ident::name() +})
				}
				_ => None,
			}
		})
		.collect();

	let sub_types_str = if !sub_types.is_empty() {
		let mut sub_types: Vec<_> = sub_types
			.into_iter()
			.flat_map(|sub_type| [sub_type, quote! {"," +}])
			.collect();
		sub_types.pop();
		quote! {+ "<" + #(#sub_types)* ">"}
	} else {
		quote! {}
	};

	// Return `impl` code
	let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

	quote! {
		const _: () = {
			use ::std::{
				mem,
				option::Option::{self, Some},
				string::String,
				stringify, vec,
			};
			use ::layout_inspect::{
				__offset_of as offset_of,
				defs::{DefStruct, DefStructField, DefType},
				Inspect, TypesCollector,
			};

			#[automatically_derived]
			impl #impl_generics Inspect for #ident #type_generics #where_clause {
				fn name() -> String {
					stringify!(#ident).to_string() #sub_types_str
				}

				// TODO: Allow deriving for unsized types
				// TODO: Deduce alignment for unsized types where possible e.g. `struct X { n: u64, s: str }`
				fn size() -> Option<usize> {
					Some(mem::size_of::<Self>())
				}

				fn align() -> Option<usize> {
					Some(mem::align_of::<Self>())
				}

				fn def(collector: &mut TypesCollector) -> DefType {
					DefType::Struct(DefStruct {
						name: Self::name(),
						size: Self::size(),
						align: Self::align(),
						fields: vec![#(#field_defs),*],
					})
				}
			}
		};
	}
}

fn get_named_field_defs(fields: FieldsNamed) -> Vec<TokenStream> {
	fields.named.iter().map(get_named_field_def).collect()
}

fn get_named_field_def(field: &Field) -> TokenStream {
	// Get field name
	let name = field.ident.as_ref().expect("Missing field name");

	// Search field attributes for `#[serde(rename = "x")]` or `#[serde(flatten)]`
	let mut ser_name: Option<String> = None;
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
											ser_name = Some(s.value());
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

	let ser_name = ser_name.unwrap_or_else(|| name.to_string());

	// Return field def
	let ty = &field.ty;
	quote! {
		DefStructField {
			name: stringify!(#name).to_string(),
			ser_name: #ser_name.to_string(),
			type_id: collector.collect::<#ty>(),
			offset: offset_of!(Self, #name),
			flatten: #flatten,
		}
	}
}
