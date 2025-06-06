# SC2IR Usage Guide

This document provides examples of how to use the SC2IR library for converting SC DSL code into IR (Intermediate Representation).

## Basic Usage

The main functionality of SC2IR is to convert SC code into IR structures. Here's a simple example:

```rust
use ir::IR;
use sc_dsl::dsl::parser::tree::Tree;
use sc2ir::IRFromSC;

// Parse SC code into a Tree structure
let sc_code = r#"
bug Cat
  gene energy Int
  gene breath Int
  ethics move
end
"#.to_string();

let tree = Tree::parse_input(sc_code).unwrap();

// Convert the Tree to IR
let ir = IR::from_sc(tree);

// Now you can work with the IR structure
println!("Number of alveolus: {}", ir.alveolus.len());
```

## Converting Types Manually

You can also use the type conversion traits directly:

```rust
use sc_dsl::dsl::ast::emitter::Specie;
use ir::Flora;
use sc2ir::FloraFromSpecie;

// Convert a Specie to Flora
let specie = Specie { raw: "Int".to_string() };
let flora = Flora::from_specie(specie);
assert_eq!(flora, Flora::Int);

// Custom types are handled as Bug types
let custom_specie = Specie { raw: "MyCustomType".to_string() };
let custom_flora = Flora::from_specie(custom_specie);
assert_eq!(custom_flora, Flora::Bug("MyCustomType".to_string()));
```

## Working with Bugs and Larvie

To convert a Bug structure to a Larvie:

```rust
use sc_dsl::dsl::ast::bug::Bug;
use sc_dsl::dsl::ast::gene::Gene;
use sc_dsl::dsl::ast::emitter::{Emitter, Specie};
use sc_dsl::dsl::ast::ethics::Ethics;
use ir::Larvie;
use sc2ir::LarvieFromBug;

// Create a Bug structure
let bug = Bug {
    specie: Specie { raw: "Cat".to_string() },
    genes: vec![
        Gene {
            tag: Emitter { raw: "energy".to_string() },
            specie: Specie { raw: "Int".to_string() },
        },
        Gene {
            tag: Emitter { raw: "breath".to_string() },
            specie: Specie { raw: "Int".to_string() },
        },
    ],
    ethics: vec![
        Ethics {
            tag: Emitter { raw: "move".to_string() },
        },
    ],
};

// Convert Bug to Larvie
let larvie = Larvie::from_bug(bug);

// Now you can work with the Larvie structure
println!("Larvie primor: {}", larvie.primor);
println!("Number of casts: {}", larvie.casts.len());
println!("Number of instincts: {}", larvie.instincts.len());
```

## Converting Genes to Casts

To convert a Gene to Casts:

```rust
use sc_dsl::dsl::ast::gene::Gene;
use sc_dsl::dsl::ast::emitter::{Emitter, Specie};
use ir::Casts;
use sc2ir::CastsFromGene;

// Create a Gene structure
let gene = Gene {
    tag: Emitter { raw: "energy".to_string() },
    specie: Specie { raw: "Int".to_string() },
};

// Convert Gene to Casts
let casts = Casts::from_gene(gene);

// Now you can work with the Casts structure
println!("Cast primor: {}", casts.primor);
println!("Cast flora: {:?}", casts.flora);
println!("Number of seals: {}", casts.seals.len());
```

## Converting Ethics to Instinct

To convert Ethics to Instinct:

```rust
use sc_dsl::dsl::ast::ethics::Ethics;
use sc_dsl::dsl::ast::emitter::Emitter;
use ir::Instinct;
use sc2ir::InstinctFromEthics;

// Create an Ethics structure
let ethics = Ethics {
    tag: Emitter { raw: "move".to_string() },
};

// Convert Ethics to Instinct
let instinct = Instinct::from_ethics(ethics);

// Now you can work with the Instinct structure
println!("Instinct echo: {}", instinct.echo);
```

## Complete Example

Here's a complete example that parses an SC file and processes the resulting IR:

```rust
use std::fs;
use ir::{IR, Flora, Alveolus};
use sc_dsl::dsl::parser::tree::Tree;
use sc2ir::{IRFromSC, FloraFromSpecie};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read SC code from a file
    let sc_code = fs::read_to_string("examples/program.sc")?;

    // Parse SC code into a Tree structure
    let tree = Tree::parse_input(sc_code)?;

    // Convert the Tree to IR
    let ir = IR::from_sc(tree);

    // Process the IR
    for (i, alveolus) in ir.alveolus.iter().enumerate() {
        match alveolus {
            Alveolus::Larvie(larvie) => {
                println!("Larvie {}: {}", i, larvie.primor);

                println!("Casts:");
                for cast in &larvie.casts {
                    println!("  - {} ({:?})", cast.primor, cast.flora);
                }

                println!("Instincts:");
                for instinct in &larvie.instincts {
                    println!("  - {}", instinct.echo);
                }
            }
        }
    }

    Ok(())
}
```

## Error Handling

The library functions don't generally return Results, as they're designed to convert valid structures. Error handling should be done at the parsing stage:

```rust
use ir::IR;
use sc_dsl::dsl::parser::tree::Tree;
use sc2ir::IRFromSC;

fn process_sc_code(code: &str) -> Result<IR, String> {
    // Try to parse the SC code
    let tree = match Tree::parse_input(code.to_string()) {
        Ok(tree) => tree,
        Err(e) => return Err(format!("Failed to parse SC code: {}", e)),
    };

    // Convert the Tree to IR (this should never fail if the Tree is valid)
    let ir = IR::from_sc(tree);

    Ok(ir)
}
```
