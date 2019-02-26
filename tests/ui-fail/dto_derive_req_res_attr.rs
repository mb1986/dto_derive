extern crate dto_derive;
use dto_derive::Dto;

#[derive(Debug, PartialEq, Dto)]
#[dto(request)]
#[dto(request)] //~ ERROR duplicate `request`/`response` attribute
struct Dto1 { }

#[derive(Debug, PartialEq, Dto)]
#[dto(request)]
#[dto(response)] //~ ERROR duplicate `request`/`response` attribute
struct Dto2 { }

#[derive(Debug, PartialEq, Dto)]
#[dto(response)]
#[dto(request)] //~ ERROR duplicate `request`/`response` attribute
struct Dto3 { }

#[derive(Debug, PartialEq, Dto)]
#[dto(response)]
#[dto(response)] //~ ERROR duplicate `request`/`response` attribute
struct Dto4 { }


fn main() { }
