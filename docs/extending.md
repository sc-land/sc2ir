# Extending SC2IR

This guide explains how to extend the SC2IR system with new conversion types or enhance existing ones.

## Adding a New Trait

To add a new conversion trait:

1. Define the trait in `src/traits/mod.rs`:

```rust
/// Trait for converting SC Component into IR Component
pub trait ComponentFromSC {
    /// Converts an SC Component to an IR Component
    ///
    /// # Parameters
    ///
    /// * `component` - The SC Component to convert
    ///
    /// # Returns
    ///
    /// An IR Component
    fn from_sc(component: SCComponent) -> IRComponent;
}
```

2. Export the trait in `lib.rs`:

```rust
pub use traits::{
    CastsFromGene,
    FloraFromSpecie,
    IRFromSC,
    InstinctFromEthics,
    LarvieFromBug,
    ComponentFromSC, // New trait
};
```

## Implementing the Trait

Create a new file in the appropriate directory:

1. For core conversions that transform major structures, add to `src/conversions/`:

```rust
// src/conversions/component.rs

use ir::IRComponent;
use sc_dsl::dsl::ast::component::SCComponent;
use crate::traits::ComponentFromSC;

impl ComponentFromSC for IRComponent {
    fn from_sc(component: SCComponent) -> Self {
        // Implementation details
        IRComponent {
            // ...
        }
    }
}
```

2. For type-specific extensions, add to `src/extensions/`:

```rust
// src/extensions/component.rs

use ir::IRComponent;
use sc_dsl::dsl::ast::component::SCComponent;
use crate::traits::ComponentFromSC;

impl ComponentFromSC for IRComponent {
    fn from_sc(component: SCComponent) -> Self {
        // Implementation details
        IRComponent {
            // ...
        }
    }
}
```

3. Update the appropriate `mod.rs` file to expose your new module:

```rust
// For conversions/mod.rs
pub mod ir;
pub mod larvie;
pub mod component; // New module

// For extensions/mod.rs
pub mod casts;
pub mod instinct;
pub mod flora;
pub mod component; // New module
```

## Adding Tests

Add tests for your new conversion in `src/lib.rs` or a separate test module:

```rust
#[cfg(test)]
mod tests {
    // ...existing tests...

    #[test]
    fn test_component_conversion() {
        let sc_component = SCComponent {
            // Test data
        };

        let ir_component = IRComponent::from_sc(sc_component.clone());

        // Assert that the conversion worked correctly
        assert_eq!(ir_component.field, sc_component.field, "Field should be correctly converted");
        // More assertions...
    }
}
```

## Enhancing Existing Conversions

To enhance an existing conversion:

1. Update the trait implementation with new functionality:

```rust
impl FloraFromSpecie for Flora {
    fn from_specie(specie: Specie) -> Self {
        match specie.raw.as_str() {
            "Int" => Flora::Int,
            "Bool" => Flora::Bool,
            "Str" => Flora::Str,
            "Float" => Flora::Float, // New type support
            "Array" => Flora::Array, // New type support
            _ => Flora::Bug(specie.raw),
        }
    }
}
```

2. Add tests for the enhanced functionality:

```rust
#[test]
fn test_enhanced_flora_conversion() {
    let test_cases = vec![
        // Existing cases...
        ("Float", Flora::Float),
        ("Array", Flora::Array),
    ];

    for (input_str, expected_flora) in test_cases {
        let specie = sc_dsl::dsl::ast::emitter::Specie {
            raw: input_str.to_string(),
        };
        let converted_flora = Flora::from_specie(specie);
        assert_eq!(
            converted_flora, expected_flora,
            "Conversion failed for: {}",
            input_str
        );
    }
}
```

3. Update documentation to reflect the changes:

```rust
/// Trait for converting SC Specie into IR Flora.
///
/// This trait handles the type conversion from SC's type system (Specie)
/// to IR's type system (Flora).
///
/// Supported types:
/// - "Int" -> Flora::Int
/// - "Bool" -> Flora::Bool
/// - "Str" -> Flora::Str
/// - "Float" -> Flora::Float
/// - "Array" -> Flora::Array
/// - Any other type -> Flora::Bug(type_name)
///
/// # Example
///
/// ```
/// use ir::Flora;
/// use sc_dsl::dsl::ast::emitter::Specie;
/// use sc2ir::FloraFromSpecie;
///
/// let specie = Specie { raw: "Float".to_string() };
/// let flora = Flora::from_specie(specie);
/// assert_eq!(flora, Flora::Float);
/// ```
pub trait FloraFromSpecie {
    // ...
}
```

## Integration with Existing Code

When adding new conversions, make sure they integrate well with the existing system:

1. If your conversion is part of a larger process, update the parent conversion to use it:

```rust
impl LarvieFromBug for Larvie {
    fn from_bug(bug: Bug) -> Larvie {
        // ...existing code...

        // Add support for new elements
        if let Some(components) = bug.components {
            for component in components {
                larvie_components.push(IRComponent::from_sc(component));
            }
        }

        Larvie {
            primor,
            casts,
            instincts,
            components: larvie_components, // New field
        }
    }
}
```

2. Update any related data structures or enums to support the new types.

## Documentation

Always update the documentation to reflect your changes:

1. Update API documentation with new traits and implementations
2. Add examples in the usage guide
3. Update PlantUML diagrams if the architecture changes
4. Add test cases that demonstrate the new functionality
