use proc_macro2::Span;
use syn::{Token, Lit, LitInt, Ident, Expr, ExprCall, ExprClosure, Path, Result, Error};
use syn::parse::{Parse, ParseStream};
use super::SpannedParse;

#[derive(Debug)]
pub(crate) struct MapStructAttr {
    pub(crate) target: Ident,
    pub(crate) source: MapStructAttrSource,
    pub(crate) span: Span,
}

#[derive(Debug)]
pub(crate) enum MapStructAttrSource {
    Field(Ident),
    ArgNo(LitInt),
    Call(ExprCall),
    Closure(ExprClosure),
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
                Expr::Path(ref expr) if expr.path.check_ident() => {
                    Ok(MapStructAttr {
                        target: left,
                        source: MapStructAttrSource::Field(expr.path.into_ident()),
                        span,
                    })
                },
                Expr::Lit(expr) => {
                    if let Lit::Int(int) = expr.lit {
                        Ok(MapStructAttr {
                            target: left,
                            source: MapStructAttrSource::ArgNo(int),
                            span,
                        })
                    } else {
                        Err(Error::new_spanned(expr, "expected argument number"))
                    }
                },
                Expr::Call(expr) => {
                    Ok(MapStructAttr {
                        target: left,
                        source: MapStructAttrSource::Call(expr),
                        span,
                    })
                },
                Expr::Closure(expr) => {
                    Ok(MapStructAttr {
                        target: left,
                        source: MapStructAttrSource::Closure(expr),
                        span,
                    })
                },
                _ => {
                    Err(Error::new_spanned(right, "unexpected expression"))
                }
            }
        } else {
            Err(Error::new_spanned(map_lit,
                "expected string literal containing mapping expression"))
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
            && self.segments[0].ident.to_string()
                .chars().next().unwrap().is_lowercase()
    }

    fn into_ident(&self) -> Ident {
        self.segments[0].ident.clone()
    }
}
