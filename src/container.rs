use itertools::Itertools;
use proc_macro2::Span;
use std::collections::{HashSet, HashMap};
use syn::{Error, Result, Ident, TypePath, Generics};
use crate::parse::{
    EntityStructAttr,
    MapStructAttr,
    MapStructAttrSource,
    SkipStructAttr,
    RequestStructAttr,
    ResponseStructAttr,
};
use crate::dto_info::{DtoInfo, DtoKind};

#[derive(Debug)]
pub(crate) struct Container<'a> {
    info: &'a DtoInfo<'a>,
    entity: Option<EntityStructAttr>,
    kind: Option<DtoKind>,
    map: HashMap<Ident, MapStructAttrSource>,
    skip: HashSet<Ident>,
}

impl<'a> Container<'a> {
    pub(crate) fn new(info: &'a DtoInfo) -> Container<'a> {
        Container {
            info,
            entity: None,
            kind: None,
            map: HashMap::new(),
            skip: HashSet::new(),
        }
    }

    pub(crate) fn set_entity(&mut self, attr: EntityStructAttr) -> Result<()> {
        if self.entity.is_none() {
            self.entity = Some(attr);
            Ok(())
        } else {
            Err(Error::new(attr.span, "already set an entity attribute"))
        }
    }

    pub(crate) fn set_map(&mut self, attr: MapStructAttr) -> Result<()> {
        if !self.map.contains_key(&attr.target) {
            self.map.insert(attr.target, attr.source);
            Ok(())
        } else {
            Err(Error::new(attr.span, format!("already mapped '{}'", attr.target.to_string())))
        }
    }

    pub(crate) fn set_skip(&mut self, attr: SkipStructAttr) -> Result<()> {
        let mut count_overlapped = 0;
        let already_skipped = attr.skips.iter()
            .filter(|ident| self.skip.contains(&ident))
            .inspect(|_| count_overlapped += 1)
            .join(", ");
        if count_overlapped == 0 {
            self.skip.extend(attr.skips);
            Ok(())
        } else {
            Err(Error::new(attr.span, format!("already skipped '{}'", already_skipped)))
        }
    }

    pub(crate) fn set_request(&mut self, attr: RequestStructAttr) -> Result<()> {
        if self.kind.is_none() {
            self.kind = Some(DtoKind::Request);
            Ok(())
        } else {
            Err(Error::new(attr.span, "already set an direction attribute"))
        }
    }

    pub(crate) fn set_response(&mut self, attr: ResponseStructAttr) -> Result<()> {
        if self.kind.is_none() {
            self.kind = Some(DtoKind::Response);
            Ok(())
        } else {
            Err(Error::new(attr.span, "already set an direction attribute"))
        }
    }

    pub(crate) fn seal(&self) -> Result<SealedContainer> {
        Ok(SealedContainer {
            entity: self.entity.clone()
                .map(|attr| attr.entity)
                .ok_or(Error::new(Span::call_site(), "attribute entity is not set"))?,
            kind: self.get_kind()?,
            mapping: self.get_mapping()?,
            skip: &self.skip,
            generics: self.info.generics,
            dto: self.info.name,
        })
    }

    fn get_kind(&self) -> Result<&DtoKind> {
        self.kind.as_ref()
            .or(self.info.kind)
            .ok_or(Error::new(Span::call_site(), "could not determine request/response"))
    }

    fn get_mapping(&self) -> Result<HashMap<Ident, Ident>> {
        Ok(self.info.fields.iter()
            // FIXME: field.unwrap()
            .filter_map(|field| Some((
                field.ident.clone().unwrap(),
                field.ident.clone().unwrap(),
            )))
            .collect())
    }
}

#[derive(Debug)]
pub(crate) struct SealedContainer<'a> {
    pub(crate) generics: &'a Generics,
    pub(crate) entity: TypePath,
    pub(crate) kind: &'a DtoKind,
    pub(crate) mapping: HashMap<Ident, Ident>,
    pub(crate) skip: &'a HashSet<Ident>,
    pub(crate) dto: &'a Ident,
}
