extern crate dto_derive;
use dto_derive::Dto;

#[derive(Debug, PartialEq, Dto)]
#[dto(map = "a: b")]
#[dto(map = "a: c")] //~ ERROR could not map already mapped field 'a'
struct Dto1 { }

fn main() { }
