use std::ops::Deref;
use proc_macro2::TokenStream;
use syn::Ident;
use quote::ToTokens;

#[derive(Debug)]
pub(crate) struct Mapping {
    pub(crate) target: MappingTarget,
    pub(crate) source: MappingSource,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) struct MappingTarget(pub(crate) Ident);

impl Deref for MappingTarget {
    type Target = Ident;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToTokens for MappingTarget {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

#[derive(Debug)]
pub(crate) enum MappingSource {
    Field(Ident),
}

impl ToTokens for MappingSource {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            MappingSource::Field(ref ident) => ident.to_tokens(tokens),
        }
    }
}
