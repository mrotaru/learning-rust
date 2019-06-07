# Chapter 14 - More About Cargo and crates.io

## Profiles

```
[profile.dev]
opt-level = 0
```
- can override in `Cargo.toml`:

```
[profile.dev]
opt-level = 1
```
## Doc Comments

```
/// Adds one
///
/// # Examples
/// ```
/// let x = 1; 
/// assert_eq!(2, my_crate::add_one(x));
/// ```
```

- such comments (`///`) will be used when generating docs with `cargo doc`
- doc comments will be interpreted as Markdown (which flavor ?)
- `cargo doc --open` will open the docs in the browser
- more sections: Panics, Errors, Safety
- tests inside doc comments will be run when `cargo test`

## Crate Comments
```
//! # My Crate
//!
//! `my_crate` is this and that
```

- this blurb will be included after the crate title

## Reexports
- can be used to "surface" deeply-nested objects, for usability/DX
- example: `pub use foo::Nested`
- these reexports will be listed in the "Reexports" section in generated docs

## Publishing
- if crate is to be published on crates.io, add some metadata
- e.g.: `authors`, `description`, `license`
- once published, cannot "undo", it is published forever
- can use `cargo yank --vers 1.0.1` to prevent this version from being depended on by new crates, but will not affect crates that already depend on it

## Workspaces

```
[workspace]

members = [
    "adder",
    "advanced-maths",
]
```

- grouping multiple crates intended to be used together
- workspaces have only one `Cargo.lock` file - all member crates use the same version of external deps
- built artifacts will be placed in the workspace `target`; member crates do not have their own `target` dirs
- member crate depending on another member crate: `advanced-maths = { path = "../advanced-maths" }`
- depended on crates are referenced as `external`: `external crate advanced_maths;`
- when running, must specify which crate: `cargo run -p adder` 
- `cargo test` will run tests from all member crates; to run from only one: `cargo test -p adder`

## Installing Binary Crates

- `cargo install ripgrep` - will put binary in `$HOME/.cargo/bin`, so make sure it's in your `$PATH`
- output form above command will actually tell you the exact file name it created