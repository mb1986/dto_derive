use dto_derive::Dto;

#[derive(Debug, PartialEq)]
struct Entity {
    pub a: String,
    pub b: i32,
    pub c: bool,
}

#[derive(Debug, PartialEq, Dto)]
#[dto(entity = "Entity")]
#[dto(request)]
struct EntityReq {
    a: String,
    b: i32,
    c: bool,
}

#[derive(Debug, PartialEq, Dto)]
#[dto(entity = "Entity")]
#[dto(response)]
struct EntityRes {
    a: String,
    b: i32,
    c: bool,
}

#[derive(Debug, PartialEq, Dto)]
#[dto(entity = "Entity")]
#[dto(request)]
struct EntityWrongResponse {
    a: String,
    b: i32,
    c: bool,
}

#[derive(Debug, PartialEq, Dto)]
#[dto(entity = "Entity")]
#[dto(response)]
struct EntityWrongRequest {
    a: String,
    b: i32,
    c: bool,
}

#[test]
fn dto_request_attr() {
    let request = EntityReq {
        a: "test_simple".to_string(),
        b: 123i32,
        c: true,
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
fn dto_response_attr() {
    let entity = Entity {
        a: "test_simple".to_string(),
        b: 123i32,
        c: true,
    };

    let expected = EntityRes {
        a: "test_simple".to_string(),
        b: 123i32,
        c: true,
    };

    let actual: EntityRes = entity.into();
    assert_eq!(expected, actual);
}

#[test]
fn dto_request_attr_response_name() {
    let request = EntityWrongResponse {
        a: "test_simple".to_string(),
        b: 123i32,
        c: true,
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
fn dto_response_attr_request_name() {
    let entity = Entity {
        a: "test_simple".to_string(),
        b: 123i32,
        c: true,
    };

    let expected = EntityWrongRequest {
        a: "test_simple".to_string(),
        b: 123i32,
        c: true,
    };

    let actual: EntityWrongRequest = entity.into();
    assert_eq!(expected, actual);
}
