extern crate dto_derive;
use dto_derive::Dto;

#[derive(Debug, PartialEq, Dto)]
#[dto(request)]
#[dto(request)] //~ ERROR already set an direction attribute
struct Dto1 { }

#[derive(Debug, PartialEq, Dto)]
#[dto(request)]
#[dto(response)] //~ ERROR already set an direction attribute
struct Dto2 { }

#[derive(Debug, PartialEq, Dto)]
#[dto(response)]
#[dto(request)] //~ ERROR already set an direction attribute
struct Dto3 { }

#[derive(Debug, PartialEq, Dto)]
#[dto(response)]
#[dto(response)] //~ ERROR already set an direction attribute
struct Dto4 { }


fn main() { }
