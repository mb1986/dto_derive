use dto_derive::Dto;

#[derive(Debug, PartialEq)]
struct Entity {
    pub a: String,
    pub b: i32,
    pub c: bool,
}

#[derive(Debug, PartialEq, Dto)]
#[dto(entity = "Entity")]
struct EntityRequest {
    a: String,
    b: i32,
    c: bool,
}

#[derive(Debug, PartialEq, Dto)]
#[dto(entity = "Entity")]
struct EntityResponse {
    a: String,
    b: i32,
    c: bool,
}

#[test]
fn dto_simple_request() {
    let request = EntityRequest {
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
fn dto_simple_response() {
    let entity = Entity {
        a: "test_simple".to_string(),
        b: 123i32,
        c: true,
    };

    let expected = EntityResponse {
        a: "test_simple".to_string(),
        b: 123i32,
        c: true,
    };

    let actual: EntityResponse = entity.into();
    assert_eq!(expected, actual);
}
