
#[derive(Debug)]
pub(crate) enum DtoAttrFor {
  Simple(TypePath),
  Complex(Vec<TypePath>),
}

impl Parse for DtoAttrFor {
  fn parse(input: ParseStream) -> Result<Self> {
    let for_token = input.parse::<Token![for]>()?;
    let objs: Punctuated<TypePath, Token![,]> = input.parse_terminated(TypePath::parse)?;
    let objs_len = objs.len();
    if objs_len == 1 {
      Ok(DtoAttrFor::Simple(objs[0].clone().into()))
    } else if objs_len > 1 {
      Ok(DtoAttrFor::Complex(objs.into_iter().collect()))
    } else {
      Err(Error::new_spanned(for_token, "expected at least one ident"))
    }
  }
}

#[derive(Debug)]
pub enum DtoAttrMap {
  Rename(Ident, Ident),
  Arg(Ident, LitInt),
  Expr(Ident, ExprCall),
}

impl Parse for DtoAttrMap {
  fn parse(input: ParseStream) -> Result<Self> {
    let map_token = input.parse::<kw::map>()?;
    let left = input.parse::<Ident>()?;
    input.parse::<Token![:]>()?;
    let right = input.parse::<Expr>()?;

    match right {
      Expr::Path(ref expr) if is_ident(&expr.path) =>
        Ok(DtoAttrMap::Rename(left, into_ident(&expr.path))),
        Expr::Lit(ref expr) => match &expr.lit {
          Lit::Int(expr) => Ok(DtoAttrMap::Arg(left, expr.clone())),
          _ => Err(Error::new_spanned(expr, "expected integer")),
        },
      Expr::Call(expr) => Ok(DtoAttrMap::Expr(left, expr)),
      _ => Err(Error::new_spanned(map_token, "wrong syntax")),
    }
  }
}

fn is_ident(path: &Path) -> bool {
  path.leading_colon.is_none()
    && path.segments.len() == 1
    && path.segments[0].arguments.is_empty()
    && path.segments[0].ident.to_string().chars().next().unwrap().is_lowercase()
}

fn into_ident(path: &Path) -> Ident {
  path.segments[0].ident.clone()
}

pub(crate) fn parse_struct_attrs<T>(attrs: &Vec<Attribute>) -> Result<Vec<StructAttr>> {
    attrs.iter().filter_map(|attr| if attr.path.is_ident("dto") {
        Some(syn::parse2(attr.tts.clone()))
    } else {
        None
    }).collect()
}

#[derive(Debug, PartialEq, Dto)]
#[dto(entity = "Entity")]
// #[dto(entity = "Entity", with = "String")]
// #[dto(entity = "Entity", with = "String, String")]
// #[dto(entity = "Entity", with = "String, String, String")]
#[dto(request)]
// #[dto(response)]
// #[dto(for Entity)]
// #[dto(for Entity, String)]
// //#[dto(entity Entity)]
// #[dto(entity = "Entity")]
// #[dto(map field_a: a)]
// #[dto(map field_b: b)]
// #[dto(mapn = "field_a: a, field_b: b")]
// #[dto(mapn = "field_a: a, field_b: 1")]
// #[dto(map = "field_a: a")]
// #[dto(map = "a: Uuid::new_v4()")]
// #[dto(map = "a: |dto| dto.b")]
// #[dto(map = "c: 1")]

#[dto(skip = "c")]

#[dto(skip = "c, d, e")]

#[derive(Debug, PartialEq, Dto)]
#[dto(entity = "Entity")]
struct EntityRequest {
    //#[dto(map field_a)]
    #[dto(map = "field_a")]
    a: String,
    //#[dto(map field_b)]
    #[dto(map = "field_b")]
    b: i32,
    #[dto(skip)]
    c: String,
}
