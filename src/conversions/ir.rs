use crate::traits::{IRFromSC, LarvieFromBug};
use ir::{Alveolus, IR, Larvie};
use sc_dsl::dsl::ast::anatomy::Anatomy;
use sc_dsl::dsl::ast::genome::Genome;
use sc_dsl::dsl::parser::tree::Tree;

/// Implementation of the IRFromSC trait for the IR struct
///
/// This implementation handles the conversion of an entire SC Tree into
/// an IR structure by processing each genome element in the tree.
impl IRFromSC for IR {
    /// Converts an SC Tree to an IR structure
    ///
    /// This function iterates through all genome elements in the SC tree and converts them
    /// to appropriate IR structures:
    /// - Bug elements are converted to Larvie and added as Alveolus
    /// - Behavior elements are not yet implemented (marked with todo!)
    ///
    /// # Parameters
    ///
    /// * `tree` - The SC AST tree to convert
    ///
    /// # Returns
    ///
    /// An IR structure containing all converted elements
    fn from_sc(tree: Tree) -> IR {
        let mut alveolus = Vec::new();

        for gene in tree.sc.fly.strand.genome {
            match gene {
                Genome::Anatomy(anatomy) => {
                    match anatomy {
                        Anatomy::Bug(bug) => {
                            // Convert Bug to Larvie and add it as an Alveolus
                            alveolus.push(Alveolus::Larvie(Larvie::from_bug(bug)));
                        }
                    }
                }
                Genome::Behavior(_) => {
                    // Not yet implemented
                    todo!("Implementar conversão de comportamento para alveolus");
                }
            }
        }

        IR { alveolus }
    }
}
