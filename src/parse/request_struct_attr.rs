use proc_macro2::Span;
use syn::{Ident, Result};
use syn::parse::ParseStream;
use super::SpannedParse;

#[derive(Debug, Clone)]
pub(crate) struct RequestStructAttr {
    pub(crate) ident: Ident,
    pub(crate) span: Span,
}

impl SpannedParse for RequestStructAttr {
    fn parse(input: ParseStream, span: Span) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(super::kw::request) {
            input.parse().map(|ident| RequestStructAttr { ident, span })
        } else {
            Err(lookahead.error())
        }
    }
}
