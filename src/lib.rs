extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Result};

use crate::container::{Container, SealedContainer};
use crate::dto_info::DtoInfo;
use crate::expand::expand;
use crate::parse::{parse_struct_attrs, StructAttr};

mod container;
mod dto_info;
mod expand;
mod helpers;
mod mapping;
mod parse;
mod spanned;

#[proc_macro_derive(Dto, attributes(dto))]
pub fn dto_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match process_dto_macro_derive(&input) {
        Ok(tokens) => tokens,
        Err(ref error) => error.to_compile_error().into(),
    }
}

fn process_dto_macro_derive(input: &DeriveInput) -> Result<TokenStream> {
    let dto_info = DtoInfo::from_derive(input)?;

    let mut cont = Container::new(&dto_info);

    parse_struct_attrs(&input.attrs, |attr| match attr {
        StructAttr::Entity(a) => cont.set_entity(a.entity, a.span),
        StructAttr::Request(a) => cont.set_request(a.span),
        StructAttr::Response(a) => cont.set_response(a.span),
        StructAttr::Map(a) => cont.add_mapping(a.mapping, a.span),
        StructAttr::Skip(a) => cont.add_skips(&a.skips, a.span),
    })?;

    Ok(expand(&cont.seal()?).into())
}
