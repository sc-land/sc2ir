/*!
# SC2IR - SC to IR Converter

SC2IR is a Rust library for converting SC DSL code into IR (Intermediate Representation).

The library provides a set of traits and implementations to handle the conversion of
various SC language constructs into their IR counterparts.

## Architecture

The library is organized around several main traits:

* `IRFromSC`: Converts a complete SC tree into an IR structure
* `LarvieFromBug`: Converts a Bug (SC) into a Larvie (IR)
* `CastsFromGene`: Converts a Gene (SC) into Casts (IR)
* `FloraFromSpecie`: Converts a Specie type (SC) into a Flora type (IR)
* `InstinctFromEthics`: Converts Ethics (SC) into Instinct (IR)

## Basic Example

```rust
use ir::IR;
use sc_dsl::dsl::parser::tree::Tree;
use sc2ir::IRFromSC;

// Parse SC code
let input = r#"bug Cat
  gene energy Int
  gene breath Int
  ethics move
end"#.to_string();
let tree = Tree::parse_input(input).unwrap();

// Convert SC to IR
let ir = IR::from_sc(tree.clone());
```

## Validation Example

```rust
use ir::Flora;
use ir::IR;
use sc_dsl::dsl::parser::tree::Tree;
use sc2ir::{FloraFromSpecie, IRFromSC};

let input = r#"bug Cat
  gene energy Int
  gene breath Int
  ethics move
end
"#.to_string();

let tree = Tree::parse_input(input).unwrap();
let ir = IR::from_sc(tree.clone());
let alveolus = ir.alveolus.first().unwrap();
let bug = match tree.sc.fly.strand.genome.first().unwrap() {
    sc_dsl::dsl::ast::genome::Genome::Anatomy(anatomy) => {
        match anatomy {
            sc_dsl::dsl::ast::anatomy::Anatomy::Bug(bug) => bug,
        }
    },
    _ => panic!("O teste espera um nó do tipo Bug"),
};
match alveolus {
    ir::Alveolus::Larvie(larvie) => {
        assert_eq!(larvie.primor, bug.specie.raw, "Primor deve ser igual ao specie do bug");
        assert_eq!(larvie.casts.len(), bug.genes.len(), "Número de casts deve ser igual ao número de genes");
        assert_eq!(larvie.instincts.len(), bug.ethics.len(), "Número de instincts deve ser igual ao número de ethics");
        for (cast, gene) in larvie.casts.iter().zip(bug.genes.iter()) {
            assert_eq!(cast.primor, gene.tag.raw, "Primor do cast deve ser igual ao tag do gene");
            assert_eq!(cast.flora, Flora::from_specie(gene.specie.clone()), "Flora deve ser convertida corretamente do specie");
            assert!(!cast.seals.is_empty(), "Cast deve ter pelo menos um seal");
        }
        for (instinct, ethics) in larvie.instincts.iter().zip(bug.ethics.iter()) {
            assert_eq!(instinct.echo, ethics.tag.raw, "Echo do instinct deve ser igual ao tag da ethics");
        }
    },
}
```
*/

/// Module containing the main conversion implementations for SC to IR transformation.
pub mod conversions;
/// Module containing type-specific extensions and conversion implementations.
pub mod extensions;
/// Module containing trait definitions for the conversion system.
pub mod traits;

// Re-export all the traits for easier access
pub use traits::{CastsFromGene, FloraFromSpecie, IRFromSC, InstinctFromEthics, LarvieFromBug};

#[cfg(test)]
mod tests {
    use ir::IR;
    use sc_dsl::dsl::parser::tree::Tree;
    use std::fs;

    use crate::{FloraFromSpecie, IRFromSC};
    use ir::Flora;

    #[test]
    fn converte_codigo_sc_para_ir() {
        let sc = fs::read_to_string("src/eg/sc.sc").unwrap();
        let tree = Tree::parse_input(sc).unwrap();
        let ir = IR::from_sc(tree.clone());
        let bug = match tree.sc.fly.strand.genome[0].clone() {
            sc_dsl::dsl::ast::genome::Genome::Anatomy(anatomy) => match anatomy {
                sc_dsl::dsl::ast::anatomy::Anatomy::Bug(bug) => bug,
            },
            sc_dsl::dsl::ast::genome::Genome::Behavior(_) => {
                panic!("o caso de uso do teste nem prevê este nó")
            }
        };

        let larvie = match ir.alveolus[0].clone() {
            ir::Alveolus::Larvie(larvie) => larvie,
        };

        assert_eq!(
            larvie.primor, bug.specie.raw,
            "Primor deve ser igual ao specie do bug"
        );
        assert_eq!(
            larvie.casts.len(),
            bug.genes.len(),
            "Número de casts deve ser igual ao número de genes"
        );
        assert_eq!(
            larvie.instincts.len(),
            bug.ethics.len(),
            "Número de instincts deve ser igual ao número de ethics"
        );

        for (cat, gene) in larvie.casts.iter().zip(bug.genes.iter()) {
            assert_eq!(
                cat.primor, gene.tag.raw,
                "Primor do cast deve ser igual ao tag do gene"
            );
            assert_eq!(
                cat.flora,
                Flora::from_specie(gene.specie.clone()),
                "Flora deve ser convertida corretamente do specie"
            );
            assert!(!cat.seals.is_empty(), "Cast deve ter pelo menos um seal");
        }

        for (instinct, ethics) in larvie.instincts.iter().zip(bug.ethics.iter()) {
            assert_eq!(
                instinct.echo, ethics.tag.raw,
                "Echo do instinct deve ser igual ao tag da ethics"
            );
        }
    }

    #[test]
    fn valida_conversao_tipos_flora() {
        let test_cases = vec![
            ("Int", Flora::Int),
            ("Bool", Flora::Bool),
            ("Str", Flora::Str),
            ("CustomType", Flora::Bug("CustomType".to_string())),
        ];

        for (input_str, expected_flora) in test_cases {
            let specie = sc_dsl::dsl::ast::emitter::Specie {
                raw: input_str.to_string(),
            };
            let converted_flora = Flora::from_specie(specie);
            assert_eq!(
                converted_flora, expected_flora,
                "Conversão falhou para: {}",
                input_str
            );
        }
    }

    #[test]
    fn valida_estrutura_completa_ir() {
        let sc = fs::read_to_string("src/eg/sc.sc").unwrap();
        let tree = Tree::parse_input(sc).unwrap();
        let ir = IR::from_sc(tree.clone());

        assert!(
            !ir.alveolus.is_empty(),
            "IR deve ter pelo menos um alveolus"
        );

        match &ir.alveolus[0] {
            ir::Alveolus::Larvie(larvie) => {
                assert!(
                    !larvie.casts.is_empty(),
                    "Larvie deve ter pelo menos um cast"
                );
                assert!(
                    !larvie.instincts.is_empty(),
                    "Larvie deve ter pelo menos um instinct"
                );

                for cast in &larvie.casts {
                    assert!(
                        !cast.seals.is_empty(),
                        "Todo cast deve ter pelo menos um seal"
                    );
                }

                for instinct in &larvie.instincts {
                    assert!(
                        !instinct.echo.is_empty(),
                        "Todo instinct deve ter um echo não vazio"
                    );
                }
            }
        }
    }
}
