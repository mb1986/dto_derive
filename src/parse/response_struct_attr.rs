use proc_macro2::Span;
use syn::Result;
use syn::parse::ParseStream;

use super::SpannedParse;

#[derive(Debug, Clone)]
pub(crate) struct ResponseStructAttr {
    pub(crate) span: Span,
}

impl SpannedParse for ResponseStructAttr {
    fn parse(input: ParseStream, span: Span) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(super::kw::response) {
            input
                .parse::<super::kw::response>()
                .map(|_| ResponseStructAttr { span })
        } else {
            Err(lookahead.error())
        }
    }
}
