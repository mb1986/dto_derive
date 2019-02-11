use syn::{DeriveInput, Ident, Field, Data, Result, Error, Generics};

#[derive(Debug)]
pub(crate) enum DtoKind {
    Request,
    Response,
}

#[derive(Debug)]
pub(crate) struct DtoInfo<'a> {
    pub(crate) dto_type: &'a Ident,
    pub(crate) kind: Option<&'a DtoKind>,
    pub(crate) fields: Vec<&'a Field>,
    pub(crate) generics: &'a Generics,
}

impl DtoInfo<'_> {
    pub(crate) fn from_derive(derive: &DeriveInput) -> Result<DtoInfo> {
        let dto_type = &derive.ident;
        let kind = get_dto_kind(dto_type);
        let fields = get_dto_fields(derive)?;
        let generics = &derive.generics;
        Ok(DtoInfo { dto_type, kind, fields, generics })
    }
}

fn get_dto_kind(ident: &Ident) -> Option<&DtoKind> {
    let ident_str = ident.to_string();
    if ident_str.ends_with("Request") {
        Some(&DtoKind::Request)
    } else if ident_str.ends_with("Response") {
        Some(&DtoKind::Response)
    } else {
        None
    }
}

fn get_dto_fields(input: &DeriveInput) -> Result<Vec<&Field>> {
    if let Data::Struct(ref data) = input.data {
        Ok(data.fields.iter().collect())
    } else {
        Err(Error::new(input.ident.span(), "expected struct"))
    }
}
