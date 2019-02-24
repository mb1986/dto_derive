use proc_macro2::Span;
use syn::parse::ParseStream;
use syn::{Ident, Result};

use super::SpannedParse;

#[derive(Debug, Clone)]
pub(crate) struct ResponseStructAttr {
    pub(crate) ident: Ident,
    pub(crate) span: Span,
}

impl SpannedParse for ResponseStructAttr {
    fn parse(input: ParseStream, span: Span) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(super::kw::response) {
            input
                .parse()
                .map(|ident| ResponseStructAttr { ident, span })
        } else {
            Err(lookahead.error())
        }
    }
}
