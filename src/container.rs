use itertools::Itertools;
use proc_macro2::Span;
use std::collections::{HashSet, HashMap};
use syn::{Error, Result, Ident, Generics};
use crate::dto_info::{DtoInfo, DtoKind};
use crate::mapping::{Mapping, MappingTarget, MappingSource};
use crate::entity::Entity;

#[derive(Debug)]
pub(crate) struct Container<'a> {
    info: &'a DtoInfo<'a>,
    entity: Option<Entity>,
    kind: Option<DtoKind>,
    mapping: HashMap<MappingTarget, MappingSource>,
    skip: HashSet<Ident>,
}

#[derive(Debug)]
pub(crate) struct SealedContainer<'a> {
    pub(crate) generics: &'a Generics,
    pub(crate) entity: &'a Entity,
    pub(crate) kind: &'a DtoKind,
    pub(crate) mapping: &'a HashMap<MappingTarget, MappingSource>,
    pub(crate) skip: &'a HashSet<Ident>,
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
            skip: HashSet::new(),
        }
    }

    pub(crate) fn set_entity(&mut self, entity: Entity, span: Span) -> Result<()> {
        if self.entity.is_none() {
            self.entity = Some(entity);
            Ok(())
        } else {
            Err(Error::new(span, "already set an entity attribute"))
        }
    }

    pub(crate) fn add_mapping(&mut self, mapping: Mapping, span: Span) -> Result<()> {
        if !self.mapping.contains_key(&mapping.target) {
            self.mapping.insert(mapping.target, mapping.source);
            Ok(())
        } else {
            Err(Error::new(span, format!("already mapped '{}'", mapping.target.to_string())))
        }
    }

    pub(crate) fn add_skips<'b, T>(&mut self, skip: &'b T, span: Span) -> Result<()>
    where
        &'b T: IntoIterator<Item = &'b Ident>,
    {
        let mut count_overlapped = 0;
        let already_skipped = skip.into_iter()
            .filter(|ident| self.skip.contains(ident))
            .inspect(|_| count_overlapped += 1)
            .join(", ");
        if count_overlapped == 0 {
            self.skip.extend(skip.into_iter().cloned());
            Ok(())
        } else {
            Err(Error::new(span, format!("already skipped '{}'", already_skipped)))
        }
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
            skip: &self.skip,
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
