use proc_macro2::Span;
use syn::Result;
use syn::parse::ParseStream;

use super::SpannedParse;

#[derive(Debug, Clone)]
pub(crate) struct RequestStructAttr {
    pub(crate) span: Span,
}

impl SpannedParse for RequestStructAttr {
    fn parse(input: ParseStream, span: Span) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(super::kw::request) {
            input
                .parse::<super::kw::request>()
                .map(|_| RequestStructAttr { span })
        } else {
            Err(lookahead.error())
        }
    }
}
