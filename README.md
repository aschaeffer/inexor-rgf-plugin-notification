# Inexor Reactive Graph Flow

| Project             | Module | Sub-Module   | Functionality                                                        | Tests                                                                                                                                                                |
|---------------------|--------|--------------|----------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reactive Graph Flow | Plugin | Notification | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-plugin-notification">](https://app.codecov.io/gh/aschaeffer/inexor-rgf-plugin-notification) |

### About Inexor

<a href="https://inexor.org/">
<img align="right" width="200" height="200" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-notification/main/docs/images/inexor_2.png">
</a>

* Inexor will be a new first-person shooter game which is based on a new octree-based game engine.
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

The plugin creates desktop notifications.

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/aschaeffer/inexor-rgf-plugin-notification/Rust">](https://github.com/aschaeffer/inexor-rgf-plugin-notification/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/aschaeffer/inexor-rgf-plugin-notification">]()
[<img src="https://img.shields.io/github/languages/code-size/aschaeffer/inexor-rgf-plugin-notification">]()
[<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-plugin-notification">](https://app.codecov.io/gh/aschaeffer/inexor-rgf-plugin-notification)

[<img src="https://img.shields.io/github/license/aschaeffer/inexor-rgf-plugin-notification">](https://github.com/aschaeffer/inexor-rgf-plugin-notification/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### Desktop Notification

<img src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-notification/main/docs/images/notification.png">

#### Platform Compatibility

| Platform | Compatibility | Notes                                                  |
|----------|---------------|--------------------------------------------------------|
| Linux    | ✓             | https://github.com/hoodie/notify-rust#linuxbsd-support |
| MacOS    | ✓             | https://github.com/hoodie/notify-rust#macos-support    |
| Windows  | ✓             | https://github.com/hoodie/notify-rust#windows-support  |

#### Entity Types

| Name                | Property | Data Type | Socket Type |
|---------------------|----------|-----------|-------------|
| DesktopNotification | show     | bool      | input       |
|                     | app_name | string    | input       |
|                     | summary  | string    | input       |
|                     | body     | string    | input       |
|                     | icon     | string    | input       |
|                     | timeout  | number    | input       |

### Thanks to

* https://github.com/hoodie/notify-rust
* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

|                                                                                                                                                                                                                               |           |                                                                   |
|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|-------------------------------------------------------------------|
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-logical/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
