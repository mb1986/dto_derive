use proc_macro2::Span;
use syn::{Token, Lit, Ident, Result, Error};
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use crate::helpers::Sequence;
use super::SpannedParse;

#[derive(Debug)]
pub(crate) struct SkipStructAttr {
    pub(crate) skips: Punctuated<Ident, Token![,]>,
    pub(crate) span: Span,
}

impl SpannedParse for SkipStructAttr {
    fn parse(input: ParseStream, span: Span) -> Result<Self> {
        input.parse::<super::kw::skip>()?;
        input.parse::<Token![=]>()?;
        let skip_lit = input.parse::<Lit>()?;
        if let Lit::Str(ref skip_names) = skip_lit {
            let skip_fields = skip_names.parse::<Sequence<Ident, Token![,]>>()?;
            if skip_fields.len() == 0 {
                Err(Error::new_spanned(skip_lit, "expected at least one field name"))
            } else {
                Ok(SkipStructAttr { skips: skip_fields.into_inner(), span })
            }
        } else {
            Err(Error::new_spanned(skip_lit,
                "expected string literal containing field names separated by comma"))
        }
    }
}
