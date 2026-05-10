use proc_macro2::Span;
use syn::parse::ParseStream;
use syn::{Attribute, Result};

mod entity_struct_attr;
pub(crate) use self::entity_struct_attr::EntityStructAttr;

mod skip_struct_attr;
pub(crate) use self::skip_struct_attr::SkipStructAttr;

mod map_struct_attr;
pub(crate) use self::map_struct_attr::MapStructAttr;

mod request_struct_attr;
pub(crate) use self::request_struct_attr::RequestStructAttr;

mod response_struct_attr;
pub(crate) use self::response_struct_attr::ResponseStructAttr;

pub(crate) mod kw {
    syn::custom_keyword!(entity);
    syn::custom_keyword!(request);
    syn::custom_keyword!(response);
    syn::custom_keyword!(map);
    syn::custom_keyword!(skip);
}

pub(crate) trait SpannedParse: Sized {
    fn parse(input: ParseStream, span: Span) -> Result<Self>;
}

#[derive(Debug)]
pub(crate) enum StructAttr {
    Entity(EntityStructAttr),
    Map(MapStructAttr),
    Skip(SkipStructAttr),
    Request(RequestStructAttr),
    Response(ResponseStructAttr),
}

impl StructAttr {
    fn parse_inner(input: ParseStream, span: Span) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::entity) {
            EntityStructAttr::parse(input, span).map(StructAttr::Entity)
        } else if lookahead.peek(kw::skip) {
            SkipStructAttr::parse(input, span).map(StructAttr::Skip)
        } else if lookahead.peek(kw::map) {
            MapStructAttr::parse(input, span).map(StructAttr::Map)
        } else if lookahead.peek(kw::request) {
            RequestStructAttr::parse(input, span).map(StructAttr::Request)
        } else if lookahead.peek(kw::response) {
            ResponseStructAttr::parse(input, span).map(StructAttr::Response)
        } else {
            Err(lookahead.error())
        }
    }
}

pub(crate) fn parse_struct_attrs<T>(attrs: &[Attribute], mut handler: T) -> Result<()>
where
    T: FnMut(StructAttr) -> Result<()>,
{
    for attr in attrs.iter().filter(|attr| attr.path().is_ident("dto")) {
        let span = attr.meta.require_list()?.delimiter.span().join();
        let parsed =
            attr.parse_args_with(|input: ParseStream| StructAttr::parse_inner(input, span))?;
        handler(parsed)?;
    }
    Ok(())
}
