use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, Ident};

#[proc_macro_derive(Inspect)]
pub fn inspect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    inspect_impl(input).into()
}

fn inspect_impl(input: DeriveInput) -> TokenStream {
    let type_name = input.ident;

    match input.data {
        Data::Struct(ref data) => derive_struct(data, type_name),
        _ => todo!(),
    }
}

fn derive_struct(data: &DataStruct, type_name: Ident) -> TokenStream {
    let field_defs = match data.fields {
        Fields::Named(ref fields) => fields.named.iter().map(|field| {
            let name = field.ident.as_ref().expect("Missing field name");
            let name_str = name.to_string();
            let ty = &field.ty;

            // TODO Remove trailing comma after last field
            quote! {
                &format!(
                    "{{\"name\":\"{}\",\"type\":\"{}\",\"offset\":{}}},",
                    #name_str,
                    <#ty as ::struct_inspect::Inspect>::name(),
                    ::struct_inspect::__offset_of!(#type_name, #name),
                )
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
        const _: () = {
            impl Inspect for #type_name {
                fn name() -> String {
                    #type_name_str.to_string()
                }

                fn kind() -> String {
                    "struct".to_string()
                }

                fn size() -> usize {
                    ::std::mem::size_of::<#type_name>()
                }

                fn align() -> usize {
                    ::std::mem::align_of::<#type_name>()
                }

                fn json() -> Option<String> {
                    Some("\"fields\":[".to_string() + #(#field_defs)+* + "]")
                }

                fn collect_child_types(types: &mut ::std::collections::HashMap<String, String>) {
                    #(#field_collect_types)*
                }
            }
        };
    }
}
