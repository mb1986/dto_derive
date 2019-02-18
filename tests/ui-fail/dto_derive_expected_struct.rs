extern crate dto_derive;
use dto_derive::Dto;

#[derive(Debug, PartialEq, Dto)]
enum EnumDto { } //~ ERROR expected struct

fn main() { }
