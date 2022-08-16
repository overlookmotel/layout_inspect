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
            quote! {
                ::struct_inspect::FieldDef {
                    name: #name_str,
                    offset: ::struct_inspect::offset_of!(#type_name, #name),
                    type_def: <#ty as ::struct_inspect::Inspect>::type_def(),
                }
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
                fn type_def() -> ::struct_inspect::TypeDef {
                    ::struct_inspect::TypeDef {
                        name: #type_name_str.to_string(),
                        kind: ::struct_inspect::TypeKind::Struct,
                        len: ::std::mem::size_of::<#type_name>(),
                        child: None,
                    }
                }
                fn fields_def() -> Option<Vec<::struct_inspect::FieldDef>> {
                    Some(vec![#(#field_defs, )*])
                }
            }
        };
    }
}
