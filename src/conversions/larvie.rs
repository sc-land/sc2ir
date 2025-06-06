use crate::traits::{CastsFromGene, InstinctFromEthics, LarvieFromBug};
use ir::{Casts, Instinct, Larvie};
use sc_dsl::dsl::ast::bug::Bug;

/// Implementation of the LarvieFromBug trait for the Larvie struct
///
/// This implementation handles the conversion from an SC Bug to an IR Larvie,
/// including the conversion of all its genes to casts and ethics to instincts.
impl LarvieFromBug for Larvie {
    /// Converts an SC Bug to an IR Larvie
    ///
    /// This function:
    /// 1. Sets the primor of the Larvie to the specie of the Bug
    /// 2. Converts all genes to casts using the CastsFromGene trait
    /// 3. Converts all ethics to instincts using the InstinctFromEthics trait
    ///
    /// # Parameters
    ///
    /// * `bug` - The SC Bug structure to convert
    ///
    /// # Returns
    ///
    /// A Larvie structure with all converted elements
    fn from_bug(bug: Bug) -> Larvie {
        // Use the specie name as the primor (identifier) for the Larvie
        let primor = bug.specie.raw.clone();

        // Initialize empty vectors for casts and instincts
        let mut casts: Vec<Casts> = vec![];
        let mut instincts: Vec<Instinct> = vec![];

        // Convert each gene to casts
        for gene in bug.genes {
            casts.push(Casts::from_gene(gene));
        }

        // Convert each ethics to instinct
        for ethics in bug.ethics {
            instincts.push(Instinct::from_ethics(ethics));
        }

        Larvie {
            primor,
            casts,
            instincts,
        }
    }
}
