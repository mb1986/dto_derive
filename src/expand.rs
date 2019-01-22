use proc_macro2::TokenStream;
use crate::SealedContainer;
use crate::dto_info::DtoKind;

mod expand_request;
use expand_request::expand_request;

mod expand_response;
use expand_response::expand_response;

pub(crate) fn expand(cont: &SealedContainer) -> TokenStream {
    match cont.kind {
        DtoKind::Request => expand_request(cont),
        DtoKind::Response => expand_response(cont),
    }
}
