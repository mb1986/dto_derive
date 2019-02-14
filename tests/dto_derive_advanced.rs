use dto_derive::Dto;

#[derive(Debug, PartialEq)]
struct Entity {
    pub abc: String,
    pub bcd: i32,
    pub cde: bool,
    pub def: i32,
    pub efg: bool,
}

#[derive(Debug, PartialEq, Dto)]
#[dto(entity = "Entity")]
#[dto(request)]
#[dto(skip = "to_skip, to_skip2")]
#[dto(map = "abc: a")]
#[dto(map = "bcd: b")]
#[dto(map = "def: xyz")]
#[dto(map = "efg: yyy")]
#[dto(skip = "d")]
struct DtoReq {
    a: String,
    b: i32,
    to_skip: String,
    to_skip2: String,
    cde: bool,
    xyz: i32,
    d: bool,
    yyy: bool,
}

#[derive(Debug, PartialEq, Dto)]
#[dto(entity = "Entity")]
#[dto(response)]
#[dto(map = "a: abc")]
#[dto(map = "bb: def")]
#[dto(map = "ccc: efg")]
struct DtoRes {
    a: String,
    bb: i32,
    ccc: bool,
}

#[test]
fn dto_advanced_request() {
    let request = DtoReq {
        a: String::from("aaa"),
        b: 111i32,
        to_skip: String::from("skip1"),
        to_skip2: String::from("skip2"),
        cde: true,
        xyz: 222i32,
        d: false,
        yyy: false,
    };

    let expected = Entity {
        abc: String::from("aaa"),
        bcd: 111i32,
        cde: true,
        def: 222i32,
        efg: false,
    };

    let actual: Entity = request.into();
    assert_eq!(expected, actual);
}

#[test]
fn dto_advanced_response() {
    let entity = Entity {
        abc: String::from("aaa"),
        bcd: 111i32,
        cde: true,
        def: 222i32,
        efg: false,
    };

    let expected = DtoRes {
        a: String::from("aaa"),
        bb: 222i32,
        ccc: false,
    };

    let actual: DtoRes = entity.into();
    assert_eq!(expected, actual);
}
