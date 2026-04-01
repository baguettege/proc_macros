# proc_macros

A playground for building and testing procedural macros in Rust.

## Crates

### `record`

`#[derive(Record)]`

A derive macro for immutable record types.

- Supports unit, tuple and named structs
- Generates a `new` constructor with all fields as parameters
- Generates an immutable getter for all fields
- Use `#[record(copy)]` on a field to return `T` instead of `&T`, requiring `T: Copy` 
