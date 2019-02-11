use proc_macro2::Span;
use syn::{Token, Lit, TypePath, Result, Error};
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use crate::helpers::Sequence;
use super::SpannedParse;
use crate::entity::{Entity, EntityType, EntityCompanionTypes};

#[derive(Debug)]
pub(crate) struct EntityStructAttr {
    pub(crate) entity: Entity,
    pub(crate) span: Span,
}

impl SpannedParse for EntityStructAttr {
    fn parse(input: ParseStream, span: Span) -> Result<Self> {
        let entity_type = parse_entity_arg(&input).map(EntityType)?;
        let companion_types = parse_companion_arg(&input)?.map(EntityCompanionTypes);
        Ok(EntityStructAttr { entity: Entity { entity_type, companion_types }, span })
    }
}

fn parse_entity_arg(input: &ParseStream) -> Result<TypePath> {
    input.parse::<super::kw::entity>()?;
    input.parse::<Token![=]>()?;
    let entity_lit = input.parse::<Lit>()?;
    if let Lit::Str(ref entity_name) = entity_lit {
        Ok(entity_name.parse::<TypePath>()?)
    } else {
        Err(Error::new_spanned(entity_lit,
            "expected string literal containing entity type"))
    }
}

fn parse_companion_arg(input: &ParseStream) -> Result<Option<Punctuated<TypePath, Token![,]>>> {
    if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
        input.parse::<super::kw::companion>()?;
        input.parse::<Token![=]>()?;
        let with_lit = input.parse::<Lit>()?;
        if let Lit::Str(ref with_names) = with_lit {
            let with_paths = with_names.parse::<Sequence<TypePath, Token![,]>>()?;
            if with_paths.len() == 0 {
                Ok(None)
            } else {
                Ok(Some(with_paths.into_inner()))
            }
        } else {
            Err(Error::new_spanned(with_lit,
                "expected string literal containing types separated by comma"))
        }
    } else {
        Ok(None)
    }
}
