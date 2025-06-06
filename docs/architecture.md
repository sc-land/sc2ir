# SC2IR Architecture Documentation

This document provides a high-level overview of the SC2IR system architecture, which is responsible for converting SC language structures into IR (Intermediate Representation).

## System Overview

The SC2IR system is designed to transform code written in SC DSL into an intermediate representation (IR) that can be used for further processing. The system follows a trait-based architecture where different components handle specific conversion tasks.

### Key Components:

- **SC DSL Side**:
  - `Tree`: The parsed syntax tree from SC code
  - `Bug`: A definition of an entity in SC
  - `Gene`: An attribute definition within a Bug
  - `Ethics`: A behavior definition within a Bug
  - `Specie`: A type specification

- **IR Side**:
  - `IR`: The top-level IR structure
  - `Larvie`: The IR representation of a Bug
  - `Casts`: The IR representation of a Gene (attribute)
  - `Flora`: The IR type system
  - `Instinct`: The IR representation of Ethics (behavior)

- **Conversion Traits**:
  - `IRFromSC`: Converts the entire SC Tree to IR
  - `LarvieFromBug`: Converts Bug entities to Larvie
  - `CastsFromGene`: Converts Gene attributes to Casts
  - `FloraFromSpecie`: Converts SC types to IR types
  - `InstinctFromEthics`: Converts Ethics behaviors to Instincts

## Architecture Diagram

```plantuml
@startuml "SC2IR - High Level Architecture"
skinparam backgroundColor white
skinparam componentStyle rectangle

package "SC DSL" {
  [Tree] as SCTree
  [Bug] as SCBug
  [Gene] as SCGene
  [Ethics] as SCEthics
  [Specie] as SCSpecie

  SCTree --> SCBug : contains
  SCBug --> SCGene : contains
  SCBug --> SCEthics : contains
  SCGene --> SCSpecie : uses
}

package "IR" {
  [IR] as IRMain
  [Larvie] as IRLarvie
  [Casts] as IRCasts
  [Flora] as IRFlora
  [Instinct] as IRInstinct

  IRMain --> IRLarvie : contains
  IRLarvie --> IRCasts : contains
  IRLarvie --> IRInstinct : contains
  IRCasts --> IRFlora : references
}

package "SC2IR (Conversion)" {
  interface "IRFromSC" as IRFromSCTrait
  interface "LarvieFromBug" as LarvieFromBugTrait
  interface "CastsFromGene" as CastsFromGeneTrait
  interface "FloraFromSpecie" as FloraFromSpecieTrait
  interface "InstinctFromEthics" as InstinctFromEthicsTrait

  IRFromSCTrait --> SCTree : input
  IRFromSCTrait --> IRMain : output

  LarvieFromBugTrait --> SCBug : input
  LarvieFromBugTrait --> IRLarvie : output

  CastsFromGeneTrait --> SCGene : input
  CastsFromGeneTrait --> IRCasts : output

  FloraFromSpecieTrait --> SCSpecie : input
  FloraFromSpecieTrait --> IRFlora : output

  InstinctFromEthicsTrait --> SCEthics : input
  InstinctFromEthicsTrait --> IRInstinct : output
}

SCTree -[dashed]-> IRMain : converted to
SCBug -[dashed]-> IRLarvie : converted to
SCGene -[dashed]-> IRCasts : converted to
SCSpecie -[dashed]-> IRFlora : converted to
SCEthics -[dashed]-> IRInstinct : converted to

@enduml
```

## Trait Structure

```plantuml
@startuml "SC2IR - Trait Structure"
skinparam backgroundColor white

interface "IRFromSC" as IRFromSCTrait {
  +from_sc(tree: Tree) -> IR
}

interface "LarvieFromBug" as LarvieFromBugTrait {
  +from_bug(bug: Bug) -> Larvie
}

interface "CastsFromGene" as CastsFromGeneTrait {
  +from_gene(gene: Gene) -> Casts
}

interface "FloraFromSpecie" as FloraFromSpecieTrait {
  +from_specie(specie: Specie) -> Flora
}

interface "InstinctFromEthics" as InstinctFromEthicsTrait {
  +from_ethics(ethics: Ethics) -> Instinct
}

class "IR" as IRClass {
}

class "Larvie" as LarvieClass {
}

class "Casts" as CastsClass {
}

class "Flora" as FloraClass {
}

class "Instinct" as InstinctClass {
}

IRClass ..|> IRFromSCTrait
LarvieClass ..|> LarvieFromBugTrait
CastsClass ..|> CastsFromGeneTrait
FloraClass ..|> FloraFromSpecieTrait
InstinctClass ..|> InstinctFromEthicsTrait

@enduml
```

## Conversion Process

```plantuml
@startuml "SC2IR - Conversion Process"
skinparam backgroundColor white

start
:Parse SC file to produce Tree;
:Call IR::from_sc(tree);

fork
  :Process each Bug in Tree;
  :Convert Bug to Larvie;
  :Add Larvie to IR.alveolus;
fork again
  :For each Gene in Bug;
  :Convert Gene to Casts;
  :Add Casts to Larvie.casts;
  :Set Seal::Vital for each Cast;
fork again
  :For each Ethics in Bug;
  :Convert Ethics to Instinct;
  :Add Instinct to Larvie.instincts;
end fork

:Return completed IR structure;
stop

@enduml
```

## Data Model

```plantuml
@startuml "SC2IR - Data Model"
skinparam backgroundColor white

package "SC DSL Types" {
  class Tree {
    +sc: SC
  }

  class Bug {
    +specie: Specie
    +genes: Vec<Gene>
    +ethics: Vec<Ethics>
  }

  class Gene {
    +tag: Emitter
    +specie: Specie
  }

  class Ethics {
    +tag: Emitter
  }

  class Specie {
    +raw: String
  }

  class Emitter {
    +raw: String
  }

  Tree o-- "1" Bug
  Bug o-- "*" Gene
  Bug o-- "*" Ethics
  Gene o-- "1" Specie
  Gene o-- "1" Emitter
  Ethics o-- "1" Emitter
}

package "IR Types" {
  class IR {
    +alveolus: Vec<Alveolus>
  }

  enum Alveolus {
    Larvie(Larvie)
  }

  class Larvie {
    +primor: String
    +casts: Vec<Casts>
    +instincts: Vec<Instinct>
  }

  class Casts {
    +primor: String
    +flora: Flora
    +seals: Vec<Seal>
  }

  enum Flora {
    Int
    Bool
    Str
    Bug(String)
  }

  enum Seal {
    Vital
  }

  class Instinct {
    +echo: String
  }

  IR o-- "*" Alveolus
  Alveolus o-- "1" Larvie
  Larvie o-- "*" Casts
  Larvie o-- "*" Instinct
  Casts o-- "1" Flora
  Casts o-- "*" Seal
}

@enduml
```

## Implementation Details

### IRFromSC

The `IRFromSC` trait is responsible for converting a whole SC `Tree` into an IR structure:

```rust
impl IRFromSC for IR {
    fn from_sc(tree: Tree) -> IR {
        let mut alveolus = Vec::new();

        for gene in tree.sc.fly.strand.genome {
            match gene {
                Genome::Anatomy(anatomy) => {
                    match anatomy {
                        Anatomy::Bug(bug) => {
                            alveolus.push(Alveolus::Larvie(Larvie::from_bug(bug)));
                        },
                    }
                },
                Genome::Behavior(_) => {
                    todo!("Implementar conversão de comportamento para alveolus");
                }
            }
        }

        IR { alveolus }
    }
}
```

### LarvieFromBug

The `LarvieFromBug` trait converts a `Bug` (from SC) to a `Larvie` (IR structure):

```rust
impl LarvieFromBug for Larvie {
    fn from_bug(bug: Bug) -> Larvie {
        let primor = bug.specie.raw.clone();
        let mut casts: Vec<Casts> = vec![];
        let mut instincts: Vec<Instinct> = vec![];

        for gene in bug.genes {
            casts.push(Casts::from_gene(gene));
        }

        for ethics in bug.ethics {
            instincts.push(Instinct::from_ethics(ethics));
        }

        Larvie { primor, casts, instincts }
    }
}
```

### CastsFromGene

The `CastsFromGene` trait converts a `Gene` (SC attribute definition) to `Casts` (IR attribute element):

```rust
impl CastsFromGene for Casts {
    fn from_gene(gene: Gene) -> Self {
        let primor = gene.tag.raw.clone();
        let flora = Flora::from_specie(gene.specie);
        let seals: Vec<Seal> = vec![Seal::Vital];
        Self { primor, flora, seals }
    }
}
```

### FloraFromSpecie

The `FloraFromSpecie` trait converts an SC `Specie` to an IR `Flora` type:

```rust
impl FloraFromSpecie for Flora {
    fn from_specie(specie: Specie) -> Self {
        match specie.raw.as_str() {
            "Int" => Flora::Int,
            "Bool" => Flora::Bool,
            "Str" => Flora::Str,
            _ => Flora::Bug(specie.raw),
        }
    }
}
```

### InstinctFromEthics

The `InstinctFromEthics` trait converts an SC `Ethics` to an IR `Instinct`:

```rust
impl InstinctFromEthics for Instinct {
    fn from_ethics(ethics: Ethics) -> Self {
        let echo = ethics.tag.raw.clone();
        Self { echo }
    }
}
```
