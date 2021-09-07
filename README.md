# Inexor Reactive Graph Flow

| Project | Module | Sub-Module | Functionality | Tests |
| --- | --- | --- | --- | --- |
| Reactive Graph Flow | Plugin | Numeric | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-plugin-numeric">](https://app.codecov.io/gh/aschaeffer/inexor-rgf-plugin-numeric) |

### About Inexor

<a href="https://inexor.org/">
<img align="right" width="200" height="200" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-numeric/main/docs/images/inexor_2.png">
</a>

* Inexor will be a new first person shooter game which is based on a new octree-based game engine.
* Inexor focuses on classic gameplay as we've seen in Cube2 or the Quake series.
* Inexor will be written from ground up new in C++17 and Rust.
* You can contribute anything you want: code, content, ideas..
* Inexor and all its content is 100% open source!

### About Inexor Reactive Graph Flow

The Inexor Reactive Graph Flow (RGF) manages reactive flows based on a graph database. The main interface is GraphQL.

* Semantic: Graph database with entities and relationships as first class citizens
* Reactive: entities and relationships are/can be reactive: If the input has been altered the entity processes its new state
* Interoperable: Use GraphQL for queries and mutations
* Extendable: Built in type system: components, entity types and relation types
* Memory efficient: Rust
* Fast: Rust
* Secure: Rust

### About this plugin

This plugin provides the type system and behaviour for numeric operations.

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/aschaeffer/inexor-rgf-plugin-numeric/Rust">](https://github.com/aschaeffer/inexor-rgf-plugin-numeric/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/aschaeffer/inexor-rgf-plugin-numeric">]()
[<img src="https://img.shields.io/github/languages/code-size/aschaeffer/inexor-rgf-plugin-numeric">]()
[<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-plugin-numeric">](https://app.codecov.io/gh/aschaeffer/inexor-rgf-plugin-numeric)

[<img src="https://img.shields.io/github/license/aschaeffer/inexor-rgf-plugin-numeric">](https://github.com/aschaeffer/inexor-rgf-plugin-numeric/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### Components

| Name | Description | Input Properties | Output Properties |
| --- | --- | --- | --- |
| numeric_gate | | lhs<br>rhs | result |
| numeric_operation | | lhs | result |

#### Entity Types

| Name | Description | Components |
| --- | --- | --- |
| abs |  | numeric_operation |
| acos |  | numeric_operation |
| asin |  | numeric_operation |
| atan |  | numeric_operation |
| atan2 |  | numeric_gate |
| cbrt |  | numeric_operation |
| ceil |  | numeric_operation |
| cos |  | numeric_operation |
| cosh |  | numeric_operation |
| exp |  | numeric_operation |
| exp2 |  | numeric_operation |
| floor |  | numeric_operation |
| fract |  | numeric_operation |
| hypot |  | numeric_gate |
| ln |  | numeric_operation |
| log |  | numeric_gate |
| log2 |  | numeric_operation |
| log10 |  | numeric_operation |
| pow |  | numeric_gate |
| recip |  | numeric_operation |
| round |  | numeric_operation |
| signum |  | numeric_operation |
| sin |  | numeric_operation |
| sinh |  | numeric_operation |
| sqrt |  | numeric_operation |
| tan |  | numeric_operation |
| tanh |  | numeric_operation |
| to_degrees |  | numeric_operation |
| to_radians |  | numeric_operation |
| trunc |  | numeric_operation |

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

| | | |
| --- | --- | --- |
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-numeric/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
