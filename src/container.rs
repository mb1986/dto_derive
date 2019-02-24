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
    fields: HashSet<Ident>,
    mapping: HashMap<MappingTarget, MappingSource>,
    skipped: HashSet<Ident>,
    mapped: HashSet<Ident>,
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
        let mut fields = HashSet::with_capacity(info.fields.len());
        fields.extend(info.fields.iter().map(|field| field.ident.clone().unwrap()));

        let mut mapping = HashMap::with_capacity(info.fields.len());
        mapping.extend(info.fields.iter().map(|field| (
            MappingTarget(field.ident.clone().unwrap()),
            MappingSource::Field(field.ident.clone().unwrap()),
        )));

        Container {
            info,
            entity: None,
            kind: None,
            fields,
            mapping,
            skipped: HashSet::new(),
            mapped: HashSet::new(),
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

    pub(crate) fn add_mapping(&mut self, mapping: Mapping, span: Span) -> Result<()> {
        if !self.mapped.contains(&mapping.target) {
            self.mapping.remove(
                &MappingTarget(
                    match mapping.source { MappingSource::Field(ref ident) => ident.clone() }
                ));
            self.mapped.insert(mapping.target.0.clone());
            self.mapping.insert(mapping.target, mapping.source);
            Ok(())
        } else {
            Err(Error::new(span, format!(
                "could not map already mapped field '{}'", mapping.target.0)))
        }
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
        let entity = self.entity.as_ref()
            .ok_or(Error::new(Span::call_site(), "attribute entity is not set"))?;
        let kind = self.get_kind()?;
        self.check_mappings(kind)?;
        Ok(SealedContainer {
            entity,
            kind,
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

    fn check_mappings(&self, kind: &DtoKind) -> Result<()> {
        match kind {
            DtoKind::Request => {
                for s in self.mapping.values().filter_map(|v|
                    match v { MappingSource::Field(ident) => Some(ident), })
                {
                    if !self.fields.contains(s) {
                        return Err(Error::new(Span::call_site(), format!(
                            "could not map non-existent field '{}'", s)));
                    }
                }
            },
            DtoKind::Response => {
                for t in self.mapping.keys() {
                    if !self.fields.contains(t) {
                        return Err(Error::new(Span::call_site(), format!(
                            "could not map non-existent field '{}'", t.0)));
                    }
                }
            },
        }
        Ok(())
    }
}
