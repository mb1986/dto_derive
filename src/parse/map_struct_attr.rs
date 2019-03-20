use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::{Error, Expr, ExprField, Ident, Lit, Path, Result, Token};

use super::SpannedParse;
use crate::mapping::{Mapping, MappingSource, MappingTarget};

#[derive(Debug)]
pub(crate) struct MapStructAttr {
    pub(crate) mapping: Mapping,
    pub(crate) span: Span,
}

impl SpannedParse for MapStructAttr {
    fn parse(input: ParseStream, span: Span) -> Result<Self> {
        input.parse::<super::kw::map>()?;
        input.parse::<Token![=]>()?;
        let map_lit = input.parse::<Lit>()?;
        if let Lit::Str(ref map_expr) = map_lit {
            struct MapStructAttrLit(Ident, Expr);
            impl Parse for MapStructAttrLit {
                fn parse(input: ParseStream) -> Result<Self> {
                    let left = input.parse::<Ident>()?;
                    input.parse::<Token![:]>()?;
                    let right = input.parse::<Expr>()?;
                    Ok(MapStructAttrLit(left, right))
                }
            }

            let MapStructAttrLit(left, right) = map_expr.parse()?;
            match right {
                Expr::Path(ref expr) if expr.path.check_ident() => Ok(MapStructAttr {
                    mapping: Mapping {
                        target: MappingTarget(left),
                        source: MappingSource::Field(expr.path.into_ident()),
                    },
                    span,
                }),
                Expr::Field(ref expr) if expr.valid() => Ok(MapStructAttr {
                    mapping: Mapping {
                        target: MappingTarget(left),
                        source: MappingSource::NestedField(expr.clone()),
                    },
                    span,
                }),
                _ => Err(Error::new_spanned(right, "unexpected expression")),
            }
        } else {
            Err(Error::new_spanned(
                map_lit,
                "expected string literal containing mapping expression",
            ))
        }
    }
}

trait FieldPath {
    fn valid(&self) -> bool;
}

impl FieldPath for ExprField {
    fn valid(&self) -> bool {
        let mut curr = self;
        loop {
            match *curr.base {
                Expr::Field(ref expr) => curr = expr,
                Expr::Path(ref expr) if expr.path.check_ident() => break true,
                _ => break false,
            }
        }
    }
}

trait IntoIdent {
    fn check_ident(&self) -> bool;
    fn into_ident(&self) -> Ident;
}

impl IntoIdent for Path {
    fn check_ident(&self) -> bool {
        self.leading_colon.is_none()
            && self.segments.len() == 1
            && self.segments[0].arguments.is_empty()
            && self.segments[0]
                .ident
                .to_string()
                .chars()
                .next()
                .unwrap()
                .is_lowercase()
    }

    fn into_ident(&self) -> Ident {
        self.segments[0].ident.clone()
    }
}
