use dto_derive::Dto;

#[derive(Debug, PartialEq)]
struct Entity {
    pub a: String,
    pub b: i32,
    pub c: bool,
}

#[derive(Debug, PartialEq, Dto)]
#[dto(entity = "Entity")]
#[dto(skip = "d")]
struct DtoSkipRequest {
    a: String,
    b: i32,
    c: bool,
    d: String,
}

#[derive(Debug, PartialEq, Dto)]
#[dto(entity = "Entity")]
#[dto(skip = "d,e,f")]
struct DtoMultiSkipOnceRequest {
    a: String,
    b: i32,
    c: bool,
    d: i32,
    e: i32,
    f: i32,
}

#[derive(Debug, PartialEq, Dto)]
#[dto(entity = "Entity")]
#[dto(skip = "d")]
#[dto(skip = "e")]
#[dto(skip = "f")]
struct DtoMultiSkipRequest {
    a: String,
    b: i32,
    c: bool,
    d: i32,
    e: i32,
    f: i32,
}

#[test]
fn dto_request_one_skip_attr() {
    let request = DtoSkipRequest {
        a: "test_simple".to_string(),
        b: 123i32,
        c: true,
        d: "skipped".to_string(),
    };

    let expected = Entity {
        a: "test_simple".to_string(),
        b: 123i32,
        c: true,
    };

    let actual: Entity = request.into();
    assert_eq!(expected, actual);
}

#[test]
fn dto_request_multi_skip_once_attr() {
    let request = DtoMultiSkipOnceRequest {
        a: "test_simple".to_string(),
        b: 123i32,
        c: true,
        d: 10i32,
        e: 10i32,
        f: 10i32,
    };

    let expected = Entity {
        a: "test_simple".to_string(),
        b: 123i32,
        c: true,
    };

    let actual: Entity = request.into();
    assert_eq!(expected, actual);
}

#[test]
fn dto_request_multi_skip_attr() {
    let request = DtoMultiSkipRequest {
        a: "test_simple".to_string(),
        b: 123i32,
        c: true,
        d: 10i32,
        e: 10i32,
        f: 10i32,
    };

    let expected = Entity {
        a: "test_simple".to_string(),
        b: 123i32,
        c: true,
    };

    let actual: Entity = request.into();
    assert_eq!(expected, actual);
}
