extern crate dto_derive;
use dto_derive::Dto;

#[derive(Debug, PartialEq, Dto)]
#[dto(skip = 123)] //~ ERROR expected string literal containing field names separated by comma
struct Dto1 { }

#[derive(Debug, PartialEq, Dto)]
#[dto(skip = "")] //~ ERROR expected at least one field name
struct Dto2 { }

#[derive(Debug, PartialEq, Dto)]
#[dto(skip = "a, b, c")] //~ ERROR field 'a' does not exist
struct Dto3 { }

#[derive(Debug, PartialEq, Dto)]
#[dto(skip = "a, b, c")] //~ ERROR field 'b' does not exist
struct Dto4 {
    a: String,
}

#[derive(Debug, PartialEq, Dto)]
#[dto(skip = "a, b, c")]
#[dto(skip = "b")] //~ ERROR already skipped 'b'
struct Dto5 {
    a: String,
    b: String,
    c: String,
}

#[derive(Debug, PartialEq, Dto)]
#[dto(skip = "a, b, b, c")]  //~ ERROR already skipped 'b'
struct Dto6 {
    a: String,
    b: String,
    c: String,
}

fn main() { }
