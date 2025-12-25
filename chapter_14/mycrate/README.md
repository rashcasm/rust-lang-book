# Chapter 14: More About Cargo and Crates.io

This chapter covers advanced Cargo features and publishing crates to [crates.io](https://crates.io/), the Rust community's package registry.

## Topics Covered

- Customizing builds with release profiles
- Publishing a crate to crates.io
- Documentation comments and generating documentation with `cargo doc`
- Organizing large projects with workspaces
- Installing binaries from crates.io with `cargo install`
- Extending Cargo with custom commands

## Example Crate: `mycrate`

The `mycrate` directory contains a simple example library crate that demonstrates:

- Writing comprehensive documentation comments
- Using `cargo doc` to generate HTML documentation
- Publishing a crate to crates.io

### Building and Running

To build the crate:

```bash
cd mycrate
cargo build
```

To run the binary (which just prints "Hello, world!"):

```bash
cargo run
```

To generate documentation:

```bash
cargo doc --open
```

This will open the generated documentation in your browser.

### Publishing to Crates.io

**Note:** Publishing requires a crates.io account and API token. The crate has already been published as `rashmin_rustbook_ch14`.

To publish your own version:

1. Create an account at [crates.io](https://crates.io/)
2. Get your API token from your account settings
3. Run `cargo login` with your token
4. Update the package name in `Cargo.toml`
5. Run `cargo publish`

### Library Functions

The crate provides simple arithmetic functions:

- `add_one(x: i32) -> i32`: Adds 1 to the given number
- `add_two(x: i32) -> i32`: Adds 2 to the given number
- `add_custom(x: i32, value: i32) -> i32`: Adds a custom value (panics if value is negative)

## Exercises

Try the following:

1. Modify the functions to work with different numeric types
2. Add more comprehensive error handling
3. Create additional documentation examples
4. Experiment with different release profiles in `Cargo.toml`