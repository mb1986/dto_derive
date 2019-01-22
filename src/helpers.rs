use std::ops::Deref;
use syn::Result;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

pub(crate) struct Sequence<T, P>(Punctuated<T, P>) where T: Parse, P: Parse;

impl<T: Parse, P: Parse> Sequence<T, P> {
    pub(crate) fn into_inner(self) -> Punctuated<T, P> {
        self.0
    }
}

impl<T: Parse, P: Parse> Parse for Sequence<T, P> {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse_terminated::<T, P>(T::parse)
            .map(Sequence)
    }
}

impl<T: Parse, P: Parse> Deref for Sequence<T, P> {
    type Target = Punctuated<T, P>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
