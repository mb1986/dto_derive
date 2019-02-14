# #[derive(Dto)]

[![Build Status](https://travis-ci.com/mb1986/dto_derive.svg?branch=develop)](https://travis-ci.com/mb1986/dto_derive)

This crate provides `Dto` derive for automatic mapping
DTOs to Entities and vice versa.

## Data-Transfer-Object Derive

- **`#[derive(Dto)]`**

  ```rust
  #[derive(Dto)]
  #[dto(entity = "Entity")]
  struct DtoRequest {
    ...
  }
  ```

## Struct Attributes

- **`#[dto(entity = "Entity")]`**

  Required attribute, has to point to an entity-structure type.

  ```rust
  impl Into<Entity> for DtoRequest {
    ...
  }
  ```

  ```rust
  impl From<Entity> for DtoResponse {
    ...
  }
  ```

- **`#[dto(entity = "Entity", with = "String, String")]`**

  Extended version of attribute with additional arguments.

  ```rust
  impl Into<Entity> for (DtoRequest, String, String) {
    ...
  }
  ```

  ```rust
  impl From<(Entity, String, String)> for DtoResponse) {
    ...
  }
  ```

- **`#[dto(request)]`**, **`#[dto(response)]`**

  Needed when dto-struct name does not end with `Request` or `Response`.

  ```rust
  #[derive(Dto)]
  #[dto(request)]
  struct FirstDto {
    ...
  }

  #[derive(Dto)]
  #[dto(response)]
  struct SecondDto {
    ...
  }
  ```

  Above code is functional equivalent to:

  ```rust
  #[derive(Dto)]
  struct FirstDtoRequest {
    ...
  }

  #[derive(Dto)]
  struct SecondDtoResponse {
    ...
  }
  ```

  <!-- ```rust
  use syn::Ident;
  ``` -->

- **`#[dto(map = "a: Uuid::new_v4()")]`**

  Assigns a value of a given expression.

  ```rust
  #[derive(Dto)]
  #[dto(entity = "Entity")]
  #[dto(map = "c: Uuid::new_v4()")]
  struct DtoRequest {
    pub a: String,
    pub b: String,
  }
  ```

  produces:

  ```rust
  impl Into<Entity> for DtoRequest {
    fn into(self) -> Entity {
      Entity {
        a: self.a,
        b: self.b,
        c: Uuid::new_v4(),
      }
    }
  }
  ```

  ```rust
  #[derive(Dto)]
  #[dto(entity = "Entity")]
  #[dto(map = "c: Uuid::new_v4()")]
  struct DtoResponse {
    pub a: String,
    pub b: String,
    pub c: Uuid,
  }
  ```

  produces:

  ```rust
  impl From<Entity> for DtoResponse {
    fn from(entity: Entity) -> Self {
      Self {
        a: entity.a,
        b: entity.b,
        c: Uuid::new_v4(),
      }
    }
  }
  ```

  <!-- ```rust
  use syn::ExprCall;
  ``` -->

- **`#[dto(map = "a: |dto| dto.b")]`**

  Assigns a value of a result of given closure.

  ```rust
  #[derive(Dto)]
  #[dto(entity = "Entity")]
  #[dto(map = "c: |ref dto| dto.c.to_hyphenated().to_string()")]
  struct DtoRequest {
    pub a: String,
    pub b: String,
    pub c: Uuid,
  }
  ```

  produces:

  ```rust
  impl Into<Entity> for DtoRequest {
    fn into(self) -> Entity {
      Entity {
        a: self.a,
        b: self.b,
        c: (|ref dto| dto.c.to_hyphenated().to_string())(self),
      }
    }
  }
  ```

  ```rust
  #[derive(Dto)]
  #[dto(entity = "Entity")]
  #[dto(map = "c: |ref entity| Uuid::parse_str(entity.c).unwarp()")]
  struct DtoResponse {
    pub a: String,
    pub b: String,
    pub c: Uuid,
  }
  ```

  produces:

  ```rust
  impl From<Entity> for DtoResponse {
    fn from(entity: Entity) -> Self {
      Self {
        a: entity.a,
        b: entity.b,
        c: (|ref entity| Uuid::parse_str(entity.c).unwarp())(entity),
      }
    }
  }
  ```

  <!-- ```rust
  use syn::ExprClosure; // features = "full"
  ``` -->

- **`#[dto(map = "c: 1")]`**

  Only for **requests**.

  ```rust
  #[derive(Dto)]
  #[dto(entity = "Entity", with = "Uuid")]
  #[dto(map = "c: 1")]
  struct DtoRequest {
    pub a: String,
    pub b: String,
  }
  ```

  produces:

  ```rust
  impl Into<Entity> for (DtoRequest, Uuid) {
    fn into(self) -> Entity {
      Entity {
        a: self.0.a,
        b: self.0.b,
        c: self.1,
      }
    }
  }
  ```

<!--
- `#[dto(for Entity)]` -> `#[dto(entity = "Entity")]`
- `#[dto(for Entity, String, String)]` -> `#[dto(entity = "Entity", args = "String, String")]`
- `#[dto(map a: b)]` -> `#[dto(map = "a: b")]`
- `#[dto(request)]`
- `#[dto(response)]`
- `#[dto(skip c)]` -> `#[dto(skip = "c")]`
- `#[dto(skip c, d, e)]` -> `#[dto(skip = "c, d, e")]`
-->

## Field Attributes

- **`#[dto(map = "a")]`**

  ```rust
  #[derive(Dto)]
  #[dto(entity = "Entity")]
  struct DtoRequest {
    #[dto(map = "c")]
    pub a: String,
    pub b: String,
  }
  ```

  produces:

  ```rust
  impl Into<Entity> for DtoRequest {
    fn into(self) -> Entity {
      Entity {
        c: self.a,
        b: self.b,
      }
    }
  }
  ```

  ```rust
  #[derive(Dto)]
  #[dto(entity = "Entity")]
  struct DtoResponse {
    #[dto(map = "c")]
    pub a: String,
    pub b: String,
  }
  ```

  produces:

  ```rust
  impl From<Entity> for DtoResponse {
    fn from(entity: Entity) -> Self {
      Self {
        a: entity.c,
        b: entity.b,
      }
    }
  }
  ```

- **`#[dto(map = 1)]`**

  Only for **responses**.

  ```rust
  #[derive(Dto)]
  #[dto(entity = "Entity", with = "Uuid")]
  struct DtoResponse {
    pub a: String,
    pub b: String,
    #[dto(map = 1)]
    pub c: Uuid,
  }
  ```

  produces:

  ```rust
  impl From<(Entity, Uuid)> for DtoResponse {
    fn from(entity: (Entity, Uuid)) -> Self {
      Self {
        a: entity.0.b,
        b: entity.0.b,
        c: entity.1,
      }
    }
  }
  ```

- **`#[dto(skip)]`**

  Only for **requests**.

  ```rust
  #[derive(Dto)]
  #[dto(entity = "Entity")]
  struct DtoRequest {
    pub a: String,
    pub b: String,
    #[dto(skip)]
    pub c: Uuid,
    pub d: String,
  }
  ```

  produces:

  ```rust
  impl Into<Entity> for DtoRequest {
    fn into(self) -> Entity {
      Entity {
        a: self.a,
        b: self.b,
        d: self.d,
      }
    }
  }
  ```

<!--
- `#[dto(map a)]` -> `#[dto(map = "a")]`
- `#[dto(skip)]`
-->
