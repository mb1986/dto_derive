# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] — 2019-03-21

### Fixed
- Pinned `compiletest_rs` dev-dependency to `=0.3.19` to restore a working
  test build after an upstream regression.

## [0.1.0] — 2019-03-19

### Added
- Initial release of the `Dto` derive macro for mapping between DTO
  structures and entities.
- `#[dto(entity = "...")]` attribute (required) declaring the entity
  type that a DTO converts to or from.
- `#[dto(request)]` and `#[dto(response)]` attributes selecting the
  conversion direction. The direction is inferred when the struct name
  ends with `Request` or `Response`.
- `#[dto(map = "a: b")]` attribute for renaming fields during
  conversion. Repeatable per struct.
- `#[dto(skip = "a, b, c")]` attribute for omitting fields during
  conversion. Valid on request DTOs only. Repeatable per struct.
- Compile-time errors for mapping a non-existent field, mapping the
  same field twice, and skipping the same field twice.

[Unreleased]: https://github.com/mb1986/dto_derive/compare/0.1.1...HEAD
[0.1.1]: https://github.com/mb1986/dto_derive/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/mb1986/dto_derive/releases/tag/0.1.0
