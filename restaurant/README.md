# Chapter 7 - Managing Growing Projects with Packages, Crates, and Modules

This chapter was almost entirely re-written, and the book is out of date: https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

## Packages and Crates
- package - can contain at most one library crate - but nr of binary crates (in a package) is unlimited
- `Cargo.toml` ⇒ package
- `src/main.rs` ⇒ binary crate with same name as the package, with `main.rs` as the root
- `src/lib.rs` ⇒ library crate with same name as the package, with `lib.rs` as the root
- when building, `rustc` uses the root as the entry point
- both `src/main.rs` and `src/lib.rs` present ⇒ two crates, both with the same name as the package
- have `src/bin` folder, with multiple `*.rs` files ⇒ each file is a binary crate; crate name is file name without ext

## Modules and Paths
- `std` is a crate containing the Rust standard library, and is composed of [multiple modules](https://doc.rust-lang.org/std/#modules), types and macros - like `fmt`, `io`, etc. 
- the `std` crate is available to all Rust modules by default.
- use `use` to bring things in scope
- `self` is current module, `super` is parent module - used for relative paths
- `crate` is current crate (root scope), can be used to compose absolute paths
- `as` can be used to name imported things - `use std::io::Result as IoResult`
```rust
use std::fmt; // std is a crate, fmt a module; contains a 'Result' type
use std::io; // also contains a Result type
// use std::io::Result; // import specific types/structs/etc; in this case not recommended - stay explicit about which Result you're reffering to
fn function1() -> fmt::Result { }
fn function2() -> io::Result<()> { }
```
- can combine `pub` and `use` - bring in scope, **and** make public: `pub use crate::front_of_house::hosting`
- nested paths for complex imports:
```rust
use std::io;
use std::cmp::Ordering;
// above two lines can be replaced with:
use std::{cmp::Ordering, io};
```
```rust
use std::io;
use std::io::Write;
// above two lines can be replaced with:
use std::io::{self, Write};
```

```rust
use std::collections::*; // brings in scope **all** public items (!)
```
- modules are resolved using module name as a basename for the file:
```rust
// will work if file structure is: src/front_of_house/hosting.rs
// also if: src/front_of_house.rs exports the `hosting` module
// of course, it will also work if `front_of_house` module is defined in the same file
pub use crate::front_of_house::hosting
```
