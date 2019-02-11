use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use crate::SealedContainer;
use crate::mapping::MappingSource;

pub(crate) fn expand_request(cont: &SealedContainer) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = cont.generics.split_for_impl();

    let dto_type = cont.dto_type;
    let companion_types = &cont.entity.companion_types;
    let dto = if cont.entity.has_companion() {
        quote! {(#dto_type, #companion_types)}
    } else {
        cont.dto_type.into_token_stream()
    };
    // let entity = cont.entity;
    let entity = &cont.entity.entity_type;
    let mappings: Vec<TokenStream> = cont.mapping.iter()
        .map(|(l, r)| {
            if let MappingSource::Field(field) = r {
                if cont.entity.has_companion() {
                    quote! { #l: (self.0).#field }
                } else {
                    quote! { #l: self.#field }
                }
            } else {
                unimplemented!()
            }
        }).collect();

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
