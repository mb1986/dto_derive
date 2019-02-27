extern crate dto_derive;
use dto_derive::Dto;

#[derive(Debug, PartialEq, Dto)]
#[dto(map = "a: b")]
#[dto(map = "a: c")] //~ ERROR cannot map already mapped field 'a'
struct Dto1 { }

#[derive(Debug, PartialEq, Dto)] //~ ERROR cannot map non-existent field 'c'
#[dto(entity = "Entity")]
#[dto(request)]
#[dto(map = "field_a: a")]
#[dto(map = "field_b: b")]
#[dto(map = "field_c: c")]
struct Dto2 {
    a: String,
    b: String,
}

#[derive(Debug, PartialEq, Dto)] //~ ERROR cannot map non-existent field 'c'
#[dto(entity = "Entity")]
#[dto(response)]
#[dto(map = "a: field_a")]
#[dto(map = "b: field_b")]
#[dto(map = "c: field_c")]
struct Dto3 {
    a: String,
    b: String,
}

fn main() { }
