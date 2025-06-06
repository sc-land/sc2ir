use crate::traits::FloraFromSpecie;
use ir::Flora;
use sc_dsl::dsl::ast::emitter::Specie;

/// Implementation of the FloraFromSpecie trait for the Flora enum
///
/// This implementation handles the conversion from SC Specie to IR Flora,
/// mapping standard types and handling custom types.
impl FloraFromSpecie for Flora {
    /// Converts an SC Specie to an IR Flora type
    ///
    /// This function maps standard types directly:
    /// - "Int" -> Flora::Int
    /// - "Bool" -> Flora::Bool
    /// - "Str" -> Flora::Str
    ///
    /// Any other type is treated as a custom Bug type with the raw name.
    ///
    /// # Parameters
    ///
    /// * `specie` - The SC Specie structure representing a type
    ///
    /// # Returns
    ///
    /// A Flora value representing the corresponding IR type
    fn from_specie(specie: Specie) -> Self {
        match specie.raw.as_str() {
            // Map standard primitive types
            "Int" => Flora::Int,
            "Bool" => Flora::Bool,
            "Str" => Flora::Str,
            // For any custom type, create a Bug Flora with the type name
            _ => Flora::Bug(specie.raw),
        }
    }
}
