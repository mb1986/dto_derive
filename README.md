# `#[derive(Dto)]`

[![Build Status](https://travis-ci.com/mb1986/dto_derive.svg?branch=master)](https://travis-ci.com/mb1986/dto_derive)
[![Current Crates.io Version](https://img.shields.io/crates/v/dto_derive.svg)](https://crates.io/crates/dto_derive)

This crate provides `Dto` derive automating the process of mapping DTOs
(Data Transfer Objects) into Entities and vice versa.
It is capable of implementing [`From`][from] or [`Into`][into] traits
for DTO structures regarding conversion direction.

Every DTO structure can act as a _request_ or a _response_,
which means that particular DTO structure can be converted
either from an Entity or into an Entity.
Therefore, a DTO which should be convertible into an Entity
is a _request_ DTO and a DTO which should be built from an Entity
is a _response_ DTO.

In addition to a simple one-to-one conversion, the crate allows
skipping particular fields or renaming them during conversion process.
More advanced features, like for example, assigning an external values
or field-level attributes are planned for next releases.

## Installation and basic usage

Add the following dependency to `Cargo.toml`:

```toml
[dependencies]
dto_derive = "0.1.1"
```

Then import `Dto` derive by:

```rust
use dto_derive::Dto;
```

And use it like so:

```rust
struct Post {
  ...
}

#[derive(Dto)]
#[dto(entity = "Post")]
struct PostResponse {
  ...
}
```

That enables you to convert `Post` into `PostResponse` in this case:

```rust
let post: Post = ...;
let post_response: PostResponse = post.into();
```

For more examples and use cases check sections below.

## Derive associated attributes

- **`#[dto(entity = "Entity")]`**

  Required attribute containing a type of a mapped entity.
  It has to be provided exactly once per DTO structure.

- **`#[dto(request)]`**, **`#[dto(response)]`**

  Optional attributes specifying a conversion direction,
  can be omitted when DTO structure name ends with `...Request`
  or `...Response`, e.g., `PostResponse`, otherwise have to be provided.

- **`#[dto(map = "a: b")]`**

  Optional attribute telling how to rename field names during conversion.
  It may be repeated for different fields.

- **`#[dto(skip = "a, b, c")]`**

  Optional attribute containing field names
  which should be omitted during conversion.
  It may contain multiple fields to skip and/or
  it may by repeated for different fields.
  **The attribute is only valid for _request_ DTOs.**

## Examples

Let's start with our `Post` entity implementation:

```rust
struct Post {
    title: String,
    body: String,
}
```

### Request DTO

In order to create a new post we may have a DTO representation:

```rust
#[derive(Dto)]
#[dto(entity = "Post")]
#[dto(request)]
#[dto(map = "body: content")]
#[dto(skip = "csrf_token")]
struct NewPost {
    title: String,
    content: String,
    csrf_token: String,
}
```

Above DTO may be converted to the `Post` entity using [`into()`][into_into]
function from [`Into`][into] trait:

```rust
let newPost = NewPost {
    title: String::from("Awesome post"),
    content: String::from("Awesome content of awesome post."),
    csrf_token: String::from("abcde"),
};

let post: Post = newPost.into();
```

It is possible because `NewPost` DTO is implementing [`Into`][into] trait
due to `Dto` derive.

### Response DTO

Response DTO may look like:

```rust
#[derive(Dto)]
#[dto(entity = "Post")]
#[dto(map = "content: body")]
struct PostResponse {
    title: String,
    content: String,
}
```

The conversion from entity to `PostResponse` DTO may be done using
[`into()`][into_into] function from [`Into`][into] trait:

```rust
let post = Post {
    title: String::from("Awesome post"),
    body: String::from("Awesome content of awesome post."),
};

let postResponse: PostResponse = post.into();
```

It is possible because `PostResponse` DTO is implementing [`From`][from] trait
due to `Dto` derive.

## Under the hood

Adding `#[derive(Dto)]` attribute means providing automatic implementation
of [`From`][from] or [`Into`][into] trait for a given structure.

Thus, for the `NewPost` DTO structure and the `Post` entity
(from previous section), below implementation will be automatically provided
(notice the _request_ nature of the `NewPost` DTO):

```rust
impl Into<Post> for NewPost {
    fn into(self) -> Post {
        Post {
            title: self.title,
            body: self.content,
        }
    }
}
```

The opposite implementation will be derived for the `PostResponse` DTO
which is in fact a _response_ DTO:

```rust
impl From<Post> for PostResponse {
    fn from(entity: Post) -> Self {
        PostResponse {
            title: entity.title,
            content: entity.body,
        }
    }
}
```

It is worth noting that a derive implementation is always provided **for** DTO
structures which allows to import entity structures from another crate
without breaking the [_orphan rule_][orphan_rule].

## License

Licensed under the MIT license ([LICENSE](LICENSE) or <https://opensource.org/licenses/MIT>).

[from]: https://doc.rust-lang.org/std/convert/trait.From.html
[into]: https://doc.rust-lang.org/std/convert/trait.Into.html
[into_into]: https://doc.rust-lang.org/std/convert/trait.Into.html#tymethod.into
[orphan_rule]: https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type
