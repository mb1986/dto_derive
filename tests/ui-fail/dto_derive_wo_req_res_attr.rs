extern crate dto_derive;
use dto_derive::Dto;

#[derive(Debug, PartialEq, Dto)] //~ ERROR could not determine request/response
#[dto(entity = "Entity")]
struct Dto { }

fn main() { }
