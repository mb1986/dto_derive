use proc_macro2::Span;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::{Error, Ident, Lit, Result, Token};

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
            let skip_fields: Punctuated<Ident, Token![,]> =
                skip_names.parse_with(Punctuated::parse_terminated)?;
            if skip_fields.is_empty() {
                Err(Error::new_spanned(
                    skip_lit,
                    "expected at least one field name",
                ))
            } else {
                Ok(SkipStructAttr {
                    skips: skip_fields,
                    span,
                })
            }
        } else {
            Err(Error::new_spanned(
                skip_lit,
                "expected string literal containing field names separated by comma",
            ))
        }
    }
}
