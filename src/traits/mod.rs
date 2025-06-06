use ir::{Casts, Flora, IR, Instinct, Larvie};
use sc_dsl::dsl::ast::bug::Bug;
use sc_dsl::dsl::ast::emitter::Specie;
use sc_dsl::dsl::ast::ethics::Ethics;
use sc_dsl::dsl::ast::gene::Gene;
use sc_dsl::dsl::parser::tree::Tree;

/// Trait for converting SC AST tree into IR structure.
///
/// This trait is the entry point for converting an entire SC syntax tree into
/// the corresponding IR structure.
///
/// # Example
///
/// ```
/// use ir::IR;
/// use sc_dsl::dsl::parser::tree::Tree;
/// use sc2ir::IRFromSC;
///
/// let sc_code = "bug Cat\n  gene energy Int\n  ethics move\nend".to_string();
/// let tree = Tree::parse_input(sc_code).unwrap();
/// let ir = IR::from_sc(tree);
/// ```
pub trait IRFromSC {
    /// Converts an SC `Tree` structure to an IR structure.
    ///
    /// # Parameters
    ///
    /// * `tree` - The SC AST tree to convert
    ///
    /// # Returns
    ///
    /// An `IR` structure containing the converted elements
    fn from_sc(tree: Tree) -> IR;
}

/// Trait for converting SC Bug into IR Larvie.
///
/// This trait handles the conversion from an SC Bug definition to an IR Larvie structure,
/// which includes converting genes to casts and ethics to instincts.
///
/// # Example
///
/// ```
/// use ir::Larvie;
/// use sc_dsl::dsl::ast::bug::Bug;
/// use sc2ir::LarvieFromBug;
///
/// // In a real scenario, you would obtain a Bug from parsing SC code
/// // and then convert it to Larvie
/// let bug = Bug::from_string("bug Cat\n  gene energy Int\n  ethics move\nend".to_string());
/// let larvie = Larvie::from_bug(bug.clone());
/// assert_eq!(larvie.primor, bug.specie.raw, "Primor should match the bug specie");
/// ```
pub trait LarvieFromBug {
    /// Converts an SC `Bug` structure to an IR `Larvie` structure.
    ///
    /// # Parameters
    ///
    /// * `bug` - The SC Bug structure to convert
    ///
    /// # Returns
    ///
    /// A `Larvie` structure containing the converted elements
    fn from_bug(bug: Bug) -> Larvie;
}

/// Trait for converting SC Gene into IR Casts.
///
/// This trait handles the conversion from a Gene (attribute definition in SC)
/// to Casts (attribute representation in IR).
///
/// # Example
///
/// ```
/// use ir::Casts;
/// use sc_dsl::dsl::ast::gene::Gene;
/// use sc2ir::CastsFromGene;
///
/// // In a real scenario, you would obtain a Gene from parsing SC code
/// // and then convert it to Casts
/// let gene = Gene::from_string("gene energy Int".to_string());
/// let casts = Casts::from_gene(gene.clone());
/// assert_eq!(casts.primor, gene.tag.raw, "Primor should match the gene tag");
/// assert_eq!(casts.flora, ir::Flora::Int, "Flora should match the gene specie");
/// ```
pub trait CastsFromGene {
    /// Converts an SC `Gene` structure to an IR `Casts` structure.
    ///
    /// # Parameters
    ///
    /// * `gene` - The SC Gene structure to convert
    ///
    /// # Returns
    ///
    /// A `Casts` structure representing the attribute in IR
    fn from_gene(gene: Gene) -> Casts;
}

/// Trait for converting SC Specie into IR Flora.
///
/// This trait handles the type conversion from SC's type system (Specie)
/// to IR's type system (Flora).
///
/// # Example
///
/// ```
/// use ir::Flora;
/// use sc_dsl::dsl::ast::emitter::Specie;
/// use sc2ir::FloraFromSpecie;
///
/// let specie = Specie { raw: "Int".to_string() };
/// let flora = Flora::from_specie(specie);
/// assert_eq!(flora, Flora::Int);
/// ```
pub trait FloraFromSpecie {
    /// Converts an SC `Specie` (type) to an IR `Flora` type.
    ///
    /// # Parameters
    ///
    /// * `specie` - The SC Specie structure representing a type
    ///
    /// # Returns
    ///
    /// A `Flora` value representing the corresponding IR type
    fn from_specie(specie: Specie) -> Flora;
}

/// Trait for converting SC Ethics into IR Instinct.
///
/// This trait handles the conversion from Ethics (behavior in SC)
/// to Instinct (instruction in IR).
///
/// # Example
///
/// ```
/// use ir::Instinct;
/// use sc_dsl::dsl::ast::ethics::Ethics;
/// use sc2ir::InstinctFromEthics;
///
/// // In a real scenario, you would obtain Ethics from parsing SC code
/// // and then convert it to Instinct
/// let ethics = Ethics::from_string("ethics move".to_string()).unwrap();
/// let instinct = Instinct::from_ethics(ethics.clone());
/// assert_eq!(ethics.tag.raw, instinct.echo);
/// ```
pub trait InstinctFromEthics {
    /// Converts an SC `Ethics` structure to an IR `Instinct` structure.
    ///
    /// # Parameters
    ///
    /// * `ethics` - The SC Ethics structure to convert
    ///
    /// # Returns
    ///
    /// An `Instinct` structure representing the behavior in IR
    fn from_ethics(ethics: Ethics) -> Instinct;
}
