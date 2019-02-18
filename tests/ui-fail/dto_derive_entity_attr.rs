extern crate dto_derive;
use dto_derive::Dto;

#[derive(Debug, PartialEq, Dto)]
#[dto(entity = 123)] //~ ERROR expected string literal containing entity type
struct Dto1 { }

#[derive(Debug, PartialEq, Dto)]
#[dto(entity = "Entity")]
#[dto(entity = "Entity")] //~ ERROR already set an entity attribute
struct Dto2 { }

fn main() { }
