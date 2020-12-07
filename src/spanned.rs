use proc_macro2::Span;
use std::ops::Deref;

pub(crate) struct Spanned<T> {
    pub(crate) inner: T,
    pub(crate) span: Span,
}

impl<T> Spanned<T> {
    pub(crate) fn new(inner: T, span: Span) -> Spanned<T> {
        Spanned { inner, span }
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
