# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [2.0.2] - 2024-07-25

## [2.0.1] - 2024-07-22

### Compatibility

- MSRV is now 1.73

### Fixes

- Future-proof unsafe code from things the compiler could do

## [2.0.0] - 2022-03-29

Note: while we intended to be "forever 1.0" to be safe to use in public APIs,
we checked our current users and it seemed safe enough to bump major to get the
benefits from some breaking changes.

### Compatibility

- `KString` is now a type alias to `KStringBase<...>` (where `...` depends on feature flags)
- `serde` is no longer a default feature
- `std` and `unsafe` are now default features
- MSRV is now 1.59

### Features

- Expose `StackString`
- `KString::try_inline` to exclusively use `StackString`
- `const fn`ed some functions
- Opt-in `forbid(unsafe_code)` by removing the default `unsafe` feature
- Allow configuring the heap string type outside of feature flags

### Fixes

- Add `#[must_use]` everywhere

### Performance

- Remove some extra bounds checks
- Speed up `clone`

## ~~[1.1.0] - 2022-03-28~~

## [1.0.6] - 2021-11-05

#### Features

- Add `FromStr`

## [1.0.5] - 2021-09-24

#### Features

- Add `max_inline` feature which is slower for smaller strings but faster for intermediate-length strings
- Added `arc` feature which is slower for smaller strings but, presumably, faster for large-enough strings

#### Fixes

- Ensure the size of KString matches String on 32-bit systems

## [1.0.4] - 2021-07-09

#### Features

- Add missing `impl From<&String> for KString`

## [1.0.3] - 2021-07-08

#### Performance

- Sped up `KString::clone()`

## [1.0.2] - 2021-07-06

#### Features

- `serde` support is now optional (still on by default)

#### Performance

- Sped up `KString::from_string` / `KStringCow::from_string`

## [1.0.1] - 2021-01-29


## [1.0.0] - 2020-07-07

<!-- next-url -->
[Unreleased]: https://github.com/cobalt-org/kstring/compare/v2.0.2...HEAD
[2.0.2]: https://github.com/cobalt-org/kstring/compare/v2.0.1...v2.0.2
[2.0.1]: https://github.com/cobalt-org/kstring/compare/v2.0.0...v2.0.1
[2.0.0]: https://github.com/cobalt-org/kstring/compare/v1.1.0...v2.0.0
[1.1.0]: https://github.com/cobalt-org/kstring/compare/v1.0.6...v1.1.0
[1.0.6]: https://github.com/cobalt-org/kstring/compare/v1.0.5...v1.0.6
[1.0.5]: https://github.com/cobalt-org/kstring/compare/v1.0.4...v1.0.5
[1.0.4]: https://github.com/cobalt-org/kstring/compare/v1.0.3...v1.0.4
[1.0.3]: https://github.com/cobalt-org/kstring/compare/v1.0.2...v1.0.3
[1.0.2]: https://github.com/cobalt-org/kstring/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/cobalt-org/kstring/compare/v1.0.0...v1.0.1
