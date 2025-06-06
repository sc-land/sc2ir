# SC2IR Coding Standards

This document outlines the coding standards and practices for the SC2IR project.

## Rust Code Formatting

- All Rust code should be formatted using `rustfmt` with the default settings.
- Run `cargo fmt` before committing changes.

## Documentation

### Comments

- Use triple-slash (`///`) comments for documenting public APIs.
- Use regular comments (`//`) for implementation notes.
- All public functions, traits, and types should have documentation comments.

### Example

```rust
/// Converts an SC Specie to an IR Flora type.
///
/// # Parameters
///
/// * `specie` - The SC Specie structure representing a type
///
/// # Returns
///
/// A Flora value representing the corresponding IR type
pub fn convert_specie_to_flora(specie: Specie) -> Flora {
    // Implementation details here
    // ...
}
```

## Error Handling

- Use `Result` for functions that can fail.
- Provide descriptive error messages.
- Consider using the `thiserror` crate for defining custom error types.

## Testing

- Write unit tests for all conversion functions.
- Include integration tests for the full conversion process.
- Aim for high test coverage, especially for the core conversion logic.

## Code Organization

- Keep implementation files focused on a single responsibility.
- Group related functionality in modules.
- Use the crate's module structure to organize code:
  - `traits/`: Trait definitions
  - `conversions/`: Core conversion implementations
  - `extensions/`: Type-specific conversion implementations

## Naming Conventions

- Use snake_case for function and variable names.
- Use CamelCase for types, traits, and enum variants.
- Use SCREAMING_SNAKE_CASE for constants.
- Prefer descriptive names over abbreviated ones.

## Performance Considerations

- Avoid unnecessary cloning of data.
- Use references where appropriate.
- Consider using iterators for processing collections.
- Profile code to identify bottlenecks for large input files.

## Dependencies

- Minimize external dependencies.
- Carefully evaluate new dependencies before adding them.
- Keep dependencies up to date.
