# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Declared minimum supported Rust version (MSRV) of 1.85.

### Changed
- Bumped the crate to Rust edition 2024.
- Upgraded `syn` from 0.15 to 2, `proc-macro2` from 0.4 to 1, and
  `quote` from 0.6 to 1, bringing transitive dependencies in line
  with the modern proc-macro ecosystem. The `Dto` derive's public
  surface is unchanged.
- Replaced the `compiletest_rs` dev-dependency with `trybuild` for
  UI tests, restoring compatibility with current stable Rust.
- Request-direction conversions are now emitted as
  `impl From<Dto> for Entity` instead of `impl Into<Entity> for Dto`,
  mirroring the response direction. The previous `Into` form was a
  workaround for pre-1.41 coherence rules made unnecessary by the
  new MSRV. Existing `dto.into()` call sites are unaffected;
  consumers can now also write `Entity::from(dto)`.

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
