/*!
 * Extensions module for type-specific conversions
 *
 * This module contains implementations of conversion traits for specific types:
 * - `casts`: Implementation of `CastsFromGene` for converting Genes to Casts
 * - `instinct`: Implementation of `InstinctFromEthics` for converting Ethics to Instinct
 * - `flora`: Implementation of `FloraFromSpecie` for converting Specie to Flora
 */

pub mod casts;
pub mod flora;
pub mod instinct;
