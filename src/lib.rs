extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident};

#[proc_macro_derive(EnumTypes)]
pub fn enum_types(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let vis = &ast.vis;
    let original_name = &ast.ident;
    let name = Ident::new(&format!("{}Type", ast.ident), ast.ident.span());

    let expanded = match ast.data {
        Data::Enum(ref data) => {
            let variants = data
                .variants
                .iter()
                .map(|var| {
                    let ident = &var.ident;
                    quote! {
                        #ident
                    }
                })
                .collect::<Vec<_>>();
            let fields = data
                .variants
                .iter()
                .map(|var| match var.fields {
                    Fields::Unit => quote! {},
                    Fields::Unnamed(_) => quote! {(_)},
                    Fields::Named(_) => quote! {{..}},
                })
                .collect::<Vec<_>>();
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
                #vis enum #name {
                    #(#variants),*
                }

                impl #original_name {
                    #vis fn get_type(&self) -> #name {
                        match self {
                            #(#original_name::#variants #fields => #name::#variants),*
                        }
                    }
                }
            }
        }
        Data::Struct(ref data) => syn::Error::new(
            data.struct_token.span,
            "EnumTypes can only be used on enums",
        )
        .to_compile_error(),
        Data::Union(ref data) => {
            syn::Error::new(data.union_token.span, "EnumTypes can only be used on enums")
                .to_compile_error()
        }
    };

    expanded.into()
}
