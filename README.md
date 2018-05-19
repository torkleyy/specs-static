# specs-static

[![Build Status][bi]][bl] [![Crates.io][ci]][cl] ![MIT/Apache][li] [![Docs.rs][di]][dl] ![LoC][lo]

[bi]: https://travis-ci.org/torkleyy/specs-static.svg?branch=master
[bl]: https://travis-ci.org/torkleyy/specs-static

[ci]: https://img.shields.io/crates/v/specs-static.svg
[cl]: https://crates.io/crates/specs-static/

[li]: https://img.shields.io/badge/license-MIT%2FApache-blue.svg

[di]: https://docs.rs/specs-static/badge.svg
[dl]: https://docs.rs/specs-static/

[lo]: https://tokei.rs/b1/github/torkleyy/specs-static?category=code

An experimental extension for [Specs](https://github.com/slide-rs/specs).
This adds component storages that are not indexed by `Entity`, but by custom ids that
are entirely managed by the user.

This can be useful for tile maps where you want each tile to have certain components. Managing
the ids manually will give you a huge performance boost here.

## Usage

Please see [the basic example](examples/basic.rs).

### Required Rust version

`1.24 stable`

## Features

* `Storage` with custom ids
* `get`, `get_mut` and `Join`
* `WorldExt` for registering these storages

## Contribution

Contribution is highly welcome! If you'd like another feature, just create an issue.
You can also help out with any issue you want to; just make sure to leave a
comment that you're working on it. If you need any help, feel free to ask!

All contributions are assumed to be dual-licensed under MIT/Apache-2.

## License

`specs-static` is distributed under the terms of both the MIT 
license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).
