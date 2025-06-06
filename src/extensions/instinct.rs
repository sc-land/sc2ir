use crate::traits::InstinctFromEthics;
use ir::Instinct;
use sc_dsl::dsl::ast::ethics::Ethics;

/// Implementation of the InstinctFromEthics trait for the Instinct struct
///
/// This implementation handles the conversion from SC Ethics to IR Instinct,
/// mapping the tag to echo.
impl InstinctFromEthics for Instinct {
    /// Converts an SC Ethics to an IR Instinct
    ///
    /// This function takes the tag from Ethics and uses it as the echo
    /// in the Instinct structure.
    ///
    /// # Parameters
    ///
    /// * `ethics` - The SC Ethics structure to convert
    ///
    /// # Returns
    ///
    /// An Instinct structure with the echo set to the ethics tag
    fn from_ethics(ethics: Ethics) -> Self {
        // Use the ethics tag as the echo (identifier) for the Instinct
        let echo = ethics.tag.raw.clone();
        Self { echo }
    }
}
