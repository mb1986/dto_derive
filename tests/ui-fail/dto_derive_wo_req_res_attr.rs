extern crate dto_derive;
use dto_derive::Dto;

#[derive(Debug, PartialEq, Dto)] //~ ERROR required `request`/`response` attribute or struct name ending with `Request`/`Response`
#[dto(entity = "Entity")]
struct Dto { }

fn main() { }
