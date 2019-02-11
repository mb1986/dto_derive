use proc_macro2::TokenStream;
use quote::quote;
use crate::SealedContainer;
use crate::mapping::MappingSource;

pub(crate) fn expand_response(cont: &SealedContainer) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = cont.generics.split_for_impl();

    let dto = cont.dto_type;
    let entity = cont.entity;
    let mappings: Vec<TokenStream> = cont.mapping.iter()
        .map(|(l, r)| {
            if let MappingSource::Field(field) = r {
                quote! { #l: entity.#field }
            } else {
                unimplemented!()
            }
        }).collect();

    let result = quote! {
        impl #impl_generics From<#entity> for #dto #ty_generics #where_clause {
            fn from(entity: #entity) -> #dto {
                #dto {
                    #(#mappings),*
                }
            }
        }
    };
    result
}
