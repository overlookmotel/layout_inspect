use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AttrStyle, Data, DataEnum, DataStruct, DeriveInput, Expr, Fields, Ident,
    Lit, Meta, NestedMeta,
};

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
    let field_defs = match data.fields {
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
            quote! {
                ::struct_inspect::defs::DefStructField {
                    name: #name_str.to_string(),
                    js_name: #js_name_str.to_string(),
                    type_name: <#ty as ::struct_inspect::Inspect>::name(),
                    offset: ::struct_inspect::__offset_of!(#type_name, #name),
                    flatten: #flatten,
                }
            }
        }),
        Fields::Unnamed(ref _fields) => todo!(),
        Fields::Unit => todo!(),
    };

    let field_collect_types = match data.fields {
        Fields::Named(ref fields) => fields.named.iter().map(|field| {
            let ty = &field.ty;
            quote! {
                <#ty as ::struct_inspect::Inspect>::collect_types(types);
            }
        }),
        Fields::Unnamed(ref _fields) => todo!(),
        Fields::Unit => todo!(),
    };

    let type_name_str = type_name.to_string();
    quote! {
        #[automatically_derived]
        impl Inspect for #type_name {
            fn name() -> String {
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
                types: &mut ::std::collections::HashMap<String, ::struct_inspect::defs::DefType>
            ) {
                #(#field_collect_types)*
            }
        }
    }
}

fn derive_enum(data: &DataEnum, type_name: Ident) -> TokenStream {
    let mut next_value: u64 = 0;
    let variant_defs = data.variants.iter().map(|variant| {
        let ty = match &variant.fields {
            Fields::Unit => quote! { None },
            Fields::Unnamed(ref fields) => {
                let unnamed = &fields.unnamed;
                assert!(unnamed.len() == 1);
                let ty = &unnamed.first().unwrap().ty;
                quote! {
                    Some(<#ty as ::struct_inspect::Inspect>::name())
                }
            }
            _ => todo!(),
        };

        let value = match &variant.discriminant {
            Some(discriminant) => match &discriminant.1 {
                Expr::Lit(expr_lit) => match &expr_lit.lit {
                    Lit::Int(int) => int.base10_parse::<u64>().unwrap(),
                    _ => todo!(),
                },
                _ => todo!(),
            },
            None => next_value,
        };
        next_value = value + 1;

        let type_name_str = variant.ident.to_string();
        quote! {
            ::struct_inspect::defs::DefEnumVariant {
                name: #type_name_str.to_string(),
                discriminant: #value,
                value_type_name: #ty,
            }
        }
    });

    let variant_collect_types = data.variants.iter().map(|variant| match variant.fields {
        Fields::Unit => quote! {},
        Fields::Unnamed(ref fields) => {
            let unnamed = &fields.unnamed;
            assert!(unnamed.len() == 1);
            let ty = &unnamed.first().unwrap().ty;
            quote! {
                <#ty as ::struct_inspect::Inspect>::collect_types(types);
            }
        }
        _ => todo!(),
    });

    let type_name_str = type_name.to_string();
    quote! {
        #[automatically_derived]
        impl Inspect for #type_name {
            fn name() -> String {
                #type_name_str.to_string()
            }

            fn def() -> ::struct_inspect::defs::DefType {
                ::struct_inspect::defs::DefType::Enum(
                    ::struct_inspect::defs::DefEnum {
                        name: Self::name(),
                        size: ::std::mem::size_of::<#type_name>(),
                        align: ::std::mem::align_of::<#type_name>(),
                        variants: vec![#(#variant_defs),*],
                    }
                )
            }

            fn collect_child_types(
                types: &mut ::std::collections::HashMap<String, ::struct_inspect::defs::DefType>
            ) {
                #(#variant_collect_types)*
            }
        }
    }
}
