extern crate dto_derive;
use dto_derive::Dto;

#[derive(Debug, PartialEq, Dto)] //~ ERROR required `entity` attribute
struct DtoRequest { }

#[derive(Debug, PartialEq, Dto)] //~ ERROR required `entity` attribute
struct DtoResponse { }

#[derive(Debug, PartialEq, Dto)] //~ ERROR required `entity` attribute
struct Dto { }

fn main() { }
