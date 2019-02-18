use proc_macro2::Span;
use std::collections::{HashSet, HashMap};
use syn::{Error, Result, Ident, Generics, TypePath};
use crate::dto_info::{DtoInfo, DtoKind};
use crate::mapping::{Mapping, MappingTarget, MappingSource};

#[derive(Debug)]
pub(crate) struct Container<'a> {
    info: &'a DtoInfo<'a>,
    entity: Option<TypePath>,
    kind: Option<DtoKind>,
    mapping: HashMap<MappingTarget, MappingSource>,
    skipped: HashSet<Ident>,
}

#[derive(Debug)]
pub(crate) struct SealedContainer<'a> {
    pub(crate) generics: &'a Generics,
    pub(crate) entity: &'a TypePath,
    pub(crate) kind: &'a DtoKind,
    pub(crate) mapping: &'a HashMap<MappingTarget, MappingSource>,
    pub(crate) dto_type: &'a Ident,
}

impl<'a> Container<'a> {
    pub(crate) fn new(info: &'a DtoInfo) -> Container<'a> {
        let mut mapping = HashMap::with_capacity(info.fields.len());
        mapping.extend(info.fields.iter().map(|field| (
            MappingTarget(field.ident.clone().unwrap()),
            MappingSource::Field(field.ident.clone().unwrap()),
        )));
        Container {
            info,
            entity: None,
            kind: None,
            mapping,
            skipped: HashSet::new(),
        }
    }

    pub(crate) fn set_entity(&mut self, entity: TypePath, span: Span) -> Result<()> {
        if self.entity.is_none() {
            self.entity = Some(entity);
            Ok(())
        } else {
            Err(Error::new(span, "already set an entity attribute"))
        }
    }

    pub(crate) fn add_mapping(&mut self, mapping: Mapping, _span: Span) -> Result<()> {
        self.mapping.remove(
            &MappingTarget(match mapping.source { MappingSource::Field(ref ident) => ident.clone() }));
        self.mapping.insert(mapping.target, mapping.source);
        Ok(())
    }

    pub(crate) fn add_skips<'b, T>(&mut self, skip: &'b T, span: Span) -> Result<()>
    where
        &'b T: IntoIterator<Item = &'b Ident>,
    {
        for s in skip {
            if self.skipped.contains(s) {
                return Err(Error::new(span, format!("already skipped '{}'", s)));
            }
            self.skipped.insert(s.clone());

            if self.mapping.remove(&MappingTarget(s.clone())).is_none() {
                return Err(Error::new(span, format!("field '{}' does not exist", s)));
            }
        }
        Ok(())
    }

    pub(crate) fn set_request(&mut self, span: Span) -> Result<()> {
        if self.kind.is_none() {
            self.kind = Some(DtoKind::Request);
            Ok(())
        } else {
            Err(Error::new(span, "already set an direction attribute"))
        }
    }

    pub(crate) fn set_response(&mut self, span: Span) -> Result<()> {
        if self.kind.is_none() {
            self.kind = Some(DtoKind::Response);
            Ok(())
        } else {
            Err(Error::new(span, "already set an direction attribute"))
        }
    }

    pub(crate) fn seal(&self) -> Result<SealedContainer> {
        Ok(SealedContainer {
            entity: self.entity.as_ref()
                .ok_or(Error::new(Span::call_site(), "attribute entity is not set"))?,
            kind: self.get_kind()?,
            mapping: &self.mapping,
            generics: self.info.generics,
            dto_type: self.info.dto_type,
        })
    }

    fn get_kind(&self) -> Result<&DtoKind> {
        self.kind.as_ref()
            .or(self.info.kind)
            .ok_or(Error::new(Span::call_site(), "could not determine request/response"))
    }
}
