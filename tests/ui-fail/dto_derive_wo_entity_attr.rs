extern crate dto_derive;
use dto_derive::Dto;

#[derive(Debug, PartialEq, Dto)] //~ ERROR attribute entity is not set
struct DtoRequest { }

#[derive(Debug, PartialEq, Dto)] //~ ERROR attribute entity is not set
struct DtoResponse { }

#[derive(Debug, PartialEq, Dto)] //~ ERROR attribute entity is not set
struct Dto { }

fn main() { }
