use crate::traits::{CastsFromGene, FloraFromSpecie};
use ir::{Casts, Flora, Seal};
use sc_dsl::dsl::ast::gene::Gene;

/// Implementation of the CastsFromGene trait for the Casts struct
///
/// This implementation handles the conversion from an SC Gene to IR Casts,
/// including type conversion from Specie to Flora.
impl CastsFromGene for Casts {
    /// Converts an SC Gene to IR Casts
    ///
    /// This function:
    /// 1. Sets the primor of the Casts to the tag of the Gene
    /// 2. Converts the specie to flora using the FloraFromSpecie trait
    /// 3. Adds a Seal::Vital to the seals list (default behavior)
    ///
    /// # Parameters
    ///
    /// * `gene` - The SC Gene structure to convert
    ///
    /// # Returns
    ///
    /// A Casts structure representing the attribute in IR
    fn from_gene(gene: Gene) -> Self {
        // Use the gene tag as the primor (identifier) for the Cast
        let primor = gene.tag.raw.clone();

        // Convert the gene's specie to flora (IR type system)
        let flora = Flora::from_specie(gene.specie);

        // Add the Vital seal to all casts by default
        let seals: Vec<Seal> = vec![Seal::Vital];

        Self {
            primor,
            flora,
            seals,
        }
    }
}
