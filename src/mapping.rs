use std::ops::Deref;
use proc_macro2::TokenStream;
use syn::{Ident, LitInt, ExprCall, ExprClosure};
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
    // Field(MappingSourceField),
    // ArgNo(MappingSourceArg),
    Field(Ident),
    ArgNo(LitInt),
    Call(ExprCall),
    Closure(ExprClosure),
}

// #[derive(Debug)]
// pub(crate) struct MappingSourceField(Ident);

// impl Deref for MappingSourceField {
//     type Target = Ident;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl ToTokens for MappingSourceField {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         self.0.to_tokens(tokens);
//     }
// }

// #[derive(Debug)]
// pub(crate) struct MappingSourceArg(LitInt);

// impl MappingSourceArg {
//     pub(crate) fn specify<T>(&self, f: T) -> TokenStream
//     where
//         T: Fn(u64) -> TokenStream,
//     {
//         f(self.0.value())
//     }
// }
