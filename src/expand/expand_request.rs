use proc_macro2::TokenStream;
use quote::{quote};
use crate::SealedContainer;

pub(crate) fn expand_request(cont: &SealedContainer) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = cont.generics.split_for_impl();

    let dto = cont.dto_type;
    let entity = cont.entity;
    let mappings: Vec<TokenStream> = cont.mapping.iter()
        .map(|(l, r)| quote! { #l: self.#r }).collect();

    let result = quote! {
        impl #impl_generics Into<#entity> for #dto #ty_generics #where_clause {
            fn into(self) -> #entity {
                #entity {
                    #(#mappings),*
                }
            }
        }
    };
    result
}
