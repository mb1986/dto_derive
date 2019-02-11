use std::ops::Deref;
use proc_macro2::TokenStream;
use syn::{TypePath, Token, punctuated::Punctuated};
use quote::{ToTokens, quote};

#[derive(Debug)]
pub(crate) struct Entity {
    pub(crate) entity_type: EntityType,
    pub(crate) companion_types: Option<EntityCompanionTypes>,
}

impl Entity {
    pub(crate) fn has_companion(&self) -> bool {
        self.companion_types.is_some()
    }
}

// impl ToTokens for Entity {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         self.entity_type.to_tokens(tokens)
//     }
// }

impl ToTokens for Entity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.companion_types {
            None => self.entity_type.0.to_tokens(tokens),
            Some(ref companion_types) => {
                let entity_type = &self.entity_type;
                tokens.extend(quote! { (#entity_type, #companion_types) });
            },
        }
    }
}

#[derive(Debug)]
pub(crate) struct EntityType(pub(crate) TypePath);

impl Deref for EntityType {
    type Target = TypePath;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToTokens for EntityType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

#[derive(Debug)]
pub(crate) struct EntityCompanionTypes(pub(crate) Punctuated<TypePath, Token![,]>);

impl Deref for EntityCompanionTypes {
    type Target = Punctuated<TypePath, Token![,]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToTokens for EntityCompanionTypes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}
