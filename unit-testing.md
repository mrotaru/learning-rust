# Testing Features in Rust

- some facilities are provided via macros, or by `cargo`'s `test` subcommand
- `#[cfg(test)]` - these modules will be compiled only when `test` config
- `test` config is added by `cargo` when running `cargo test`

## Unit Tests
- in the same file as the tested code, as a module marked with `#[cfg(test)]`
- code marked with `#[cfg(test)]` is not included in the generated binaries
- have access to private methods

## Integration Tests
- `tests` directory is special - `cargo` will look there for integration tests
- each file in `tests` is compiled as a separate crate, only when running `cargo test`
- to be tested, crates must be imported as usual; `cargo` does not auto-import
- no need to annotate code in `tests` with `#[cfg(test)]`
- put helpers in `/tests/common/mod.rs`; otherwise, `/tests/common.rs` will be treated as an integration test
- binary crates with only `/src/main.rs` cannot have integration tests; put functionality in `lib.rs` and test that

```
$ cargo test --help
cargo-test 
Execute all unit and integration tests of a local package

USAGE:
    cargo test [OPTIONS] [TESTNAME] [-- <args>...]

OPTIONS:
        --lib                       Test only this package's library
        --bin <NAME>...             Test only the specified binary
        --bins                      Test all binaries
        --example <NAME>...         Test only the specified example
        --examples                  Test all examples
        --test <NAME>...            Test only the specified test target
        --tests                     Test all tests
        --bench <NAME>...           Test only the specified bench target
        --benches                   Test all benches
        --all-targets               Test all targets
        --doc                       Test only this library's documentation
        --no-run                    Compile, but don't run tests
        --no-fail-fast              Run all tests regardless of failure
    -p, --package <SPEC>...         Package to run tests for
        --all                       Test all packages in the workspace
        --exclude <SPEC>...         Exclude packages from the test
    -j, --jobs <N>                  Number of parallel jobs, defaults to # of CPUs
        --release                   Build artifacts in release mode, with optimizations
        --features <FEATURES>       Space-separated list of features to activate
        --all-features              Activate all available features
        --no-default-features       Do not activate the `default` feature
        --target <TRIPLE>           Build for the target triple
        --target-dir <DIRECTORY>    Directory for all generated artifacts
        --manifest-path <PATH>      Path to Cargo.toml
        --message-format <FMT>      Error format [default: human]  [possible values: human, json, short]
    -v, --verbose                   Use verbose output (-vv very verbose/build.rs output)
    -q, --quiet                     No output printed to stdout
        --color <WHEN>              Coloring: auto, always, never
        --frozen                    Require Cargo.lock and cache are up to date
        --locked                    Require Cargo.lock is up to date
    -Z <FLAG>...                    Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
    -h, --help                      Prints help information

ARGS:
    <TESTNAME>    If specified, only run tests containing this string in their names
    <args>...     Arguments for the test binary

The test filtering argument `TESTNAME` and all the arguments following the
two dashes (`--`) are passed to the test binaries and thus to libtest
(rustc's built in unit-test and micro-benchmarking framework).  If you're
passing arguments to both Cargo and the binary, the ones after `--` go to the
binary, the ones before go to Cargo.  For details about libtest's arguments see
the output of `cargo test -- --help`.  As an example, this will run all
tests with `foo` in their name on 3 threads in parallel:

    cargo test foo -- --test-threads 3

If the --package argument is given, then SPEC is a package id specification
which indicates which package should be tested. If it is not given, then the
current package is tested. For more information on SPEC and its format, see the
`cargo help pkgid` command.

All packages in the workspace are tested if the `--all` flag is supplied. The
`--all` flag is automatically assumed for a virtual manifest.
Note that `--exclude` has to be specified in conjunction with the `--all` flag.

The --jobs argument affects the building of the test executable but does
not affect how many jobs are used when running the tests. The default value
for the --jobs argument is the number of CPUs. If you want to control the
number of simultaneous running test cases, pass the `--test-threads` option
to the test binaries:

    cargo test -- --test-threads=1

Compilation can be configured via the `test` profile in the manifest.

By default the rust test harness hides output from test execution to
keep results readable. Test output can be recovered (e.g. for debugging)
by passing `--nocapture` to the test binaries:

    cargo test -- --nocapture

To get the list of all options available for the test binaries use this:

    cargo test -- --help
```