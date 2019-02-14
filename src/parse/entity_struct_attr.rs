use proc_macro2::Span;
use syn::{Token, Lit, TypePath, Result, Error};
use syn::parse::ParseStream;
use super::SpannedParse;

#[derive(Debug)]
pub(crate) struct EntityStructAttr {
    pub(crate) entity: TypePath,
    pub(crate) span: Span,
}

impl SpannedParse for EntityStructAttr {
    fn parse(input: ParseStream, span: Span) -> Result<Self> {
        let entity = parse_entity_arg(&input)?;
        Ok(EntityStructAttr { entity, span })
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
