use dto_derive::Dto;

#[derive(Debug, PartialEq)]
struct Entity {
    pub field_a: String,
    pub field_b: i32,
    pub field_c: bool,
}

#[derive(Debug, PartialEq, Dto)]
#[dto(entity = "Entity")]
#[dto(map = "field_a: a")]
#[dto(map = "field_b: b")]
#[dto(map = "field_c: c")]
struct DtoRequest {
    a: String,
    b: i32,
    c: bool,
}

#[derive(Debug, PartialEq, Dto)]
#[dto(entity = "Entity")]
#[dto(map = "a: field_a")]
#[dto(map = "b: field_b")]
#[dto(map = "c: field_c")]
struct DtoResponse {
    a: String,
    b: i32,
    c: bool,
}

#[test]
fn dto_map_request() {
    let request = DtoRequest {
        a: "test_simple".to_string(),
        b: 123i32,
        c: true,
    };

    let expected = Entity {
        field_a: "test_simple".to_string(),
        field_b: 123i32,
        field_c: true,
    };

    let actual: Entity = request.into();
    assert_eq!(expected, actual);
}

#[test]
fn dto_map_response() {
    let entity = Entity {
        field_a: "test_simple".to_string(),
        field_b: 123i32,
        field_c: true,
    };

    let expected = DtoResponse {
        a: "test_simple".to_string(),
        b: 123i32,
        c: true,
    };

    let actual: DtoResponse = entity.into();
    assert_eq!(expected, actual);
}
