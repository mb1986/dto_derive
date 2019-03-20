use dto_derive::Dto;

#[derive(Debug, PartialEq)]
struct DeepNestedEntity {
    pub da: i32,
}

#[derive(Debug, PartialEq)]
struct NestedEntity {
    pub na: String,
    pub nb: DeepNestedEntity,
    pub nc: bool,
}

#[derive(Debug, PartialEq)]
struct Entity {
    pub pa: String,
    pub pb: NestedEntity,
    pub pc: bool,
}

// #[derive(Debug, PartialEq, Dto)]
// #[dto(entity = "Entity")]
// #[dto(map = "field_a: a")]
// #[dto(map = "field_b: b")]
// #[dto(map = "field_c: c")]
// struct DtoRequest {
//     pa: String,
//     b1: String,
//     b2: i32,
//     b3: bool,
//     pc: bool,
// }

#[derive(Debug, PartialEq, Dto)]
#[dto(entity = "Entity")]
#[dto(map = "b1: pb.na")]
#[dto(map = "b2: pb.nb.da")]
#[dto(map = "b3: pb.nc")]
struct DtoResponse {
    pa: String,
    b1: String,
    b2: i32,
    b3: bool,
    pc: bool,
}

// #[test]
// fn dto_map_request() {
//     let request = DtoRequest {
//         a: "test_simple".to_string(),
//         b: 123i32,
//         c: true,
//     };

//     let expected = Entity {
//         field_a: "test_simple".to_string(),
//         field_b: 123i32,
//         field_c: true,
//     };

//     let actual: Entity = request.into();
//     assert_eq!(expected, actual);
// }

#[test]
fn dto_map_response() {
    let entity = Entity {
        pa: "test_simple".to_string(),
        pb: NestedEntity {
            na: String::from("nested"),
            nb: DeepNestedEntity{ da: 223i32 },
            nc: false,
        },
        pc: true,
    };

    let expected = DtoResponse {
        pa: "test_simple".to_string(),
        b1: "nested".to_string(),
        b2: 223i32,
        b3: false,
        pc: true,
    };

    let actual: DtoResponse = entity.into();
    assert_eq!(expected, actual);
}
