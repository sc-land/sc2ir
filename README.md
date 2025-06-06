# SCIR - Sistema de Conversão IR

Este projeto implementa a conversão completa entre o DSL `sc_dsl` e o IR (Intermediate Representation) `ir`. O SCIR permite traduzir código SC (Smart Contracts) para uma representação intermediária otimizada para execução.

## 🚀 Instalação e Uso Rápido

### Adicionando ao Cargo.toml

```toml
[dependencies]
sc2ir = { path = "/caminho/para/sc2ir" }
ir = { path = "/caminho/para/ir" }
sc_dsl = { path = "/caminho/para/dsl" }
```

### Exemplo de Uso Básico

```rust
use ir::IR;
use sc_dsl::dsl::parser::tree::Tree;
use sc2ir::IRFromSC;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Código SC de exemplo
    let sc_code = r#"
    bug Cat
      gene energy Int
      gene breath Int
      ethics move
    end
    "#.to_string();

    // Análise do código SC
    let tree = Tree::parse_input(sc_code)?;

    // Conversão para IR
    let ir = IR::from_sc(tree);

    println!("Conversão concluída com sucesso!");

    Ok(())
}
```

### Executando Testes

```bash
# Executa todos os testes unitários e de documentação
cargo test
```

## ✨ Funcionalidades

- ✅ **Conversão completa de AST para IR**: Transforma árvores sintáticas do SC DSL em representação intermediária
- ✅ **Conversão de tipos**: Mapeamento automático de tipos SC para tipos IR
- ✅ **Validação robusta**: Testes abrangentes garantem a integridade das conversões
- ✅ **Arquitetura extensível**: Sistema modular facilita adição de novas conversões

## 📚 Documentação

Documentação detalhada está disponível no diretório `/docs`:

1. [Manifesto e Visão](docs/manifesto.md)
   - Visão geral do projeto
   - Objetivos e princípios
   - Roadmap de desenvolvimento

2. [Arquitetura](docs/architecture.md)
   - Visão geral da arquitetura do sistema
   - Diagramas de classes e componentes
   - Fluxo de dados e processos de conversão

3. [Guia de Uso](docs/usage.md)
   - Como usar a biblioteca SC2IR
   - Exemplos de código para casos comuns
   - Tratamento de erros

3. [Padrões de Codificação](docs/coding_standards.md)
   - Diretrizes de formatação de código
   - Práticas de documentação
   - Abordagem para tratamento de erros
   - Requisitos de testes

4. [Guia de Contribuição](docs/contributing.md)
   - Como contribuir para o projeto
   - Fluxo de trabalho de desenvolvimento
   - Diretrizes para pull requests
   - Adicionando novos tipos de conversão

5. [Estendendo SC2IR](docs/extending.md)
   - Adicionando novos traits de conversão
   - Implementando funcionalidades de traits
   - Integrando com código existente
   - Testando novas conversões

6. [Visualizando Diagramas](docs/visualizing_diagrams.md)
   - Como visualizar e gerar diagramas PlantUML
   - Opções de visualização online e local
   - Exportando diagramas para diferentes formatos

### 📊 Diagramas PlantUML

A documentação inclui diagramas PlantUML para visualização da arquitetura:

- **Arquitetura de Alto Nível** (`high_level_architecture.puml`): Visão geral dos componentes e suas interações
- **Estrutura de Traits** (`trait_structure.puml`): Organização dos traits e suas implementações
- **Processo de Conversão** (`conversion_process.puml`): Fluxo de trabalho do processo de conversão
- **Modelo de Dados** (`data_model.puml`): Relações entre classes do SC e do IR

Para visualizar os diagramas, consulte o guia [Visualizando Diagramas](docs/visualizing_diagrams.md).

#### Diagrama Simplificado

```
         +-------------+
         |     SC      |
         |    Code     |
         +------+------+
                |
                v
    +---------------------+
    |      SC Tree        |
    | (Parsing/AST)       |
    +----------+----------+
               |
      +--------v---------+
      |    IRFromSC      |
      | (Trait System)   |
      +--------+---------+
               |
     +---------v----------+
     |    LarvieFromBug   |
     |    CastsFromGene   |
     | InstinctFromEthics |
     |  FloraFromSpecie   |
     +---------+----------+
               |
               v
      +------------------+
      |    IR Output     |
      +------------------+
```

## 📁 Estrutura do Projeto

```
src/
├── lib.rs              # Módulo principal com re-exportações
├── traits/             # Definição dos traits de conversão
│   └── mod.rs         # Traits: IRFromSC, LarvieFromBug, etc.
├── conversions/        # Implementações das conversões principais
│   ├── mod.rs         # Módulo principal
│   ├── ir.rs          # Conversão Tree → IR
│   └── larvie.rs      # Conversão Bug → Larvie
├── extensions/         # Extensões para tipos específicos
│   ├── mod.rs         # Módulo principal
│   ├── casts.rs       # Conversão Gene → Casts
│   ├── flora.rs       # Conversão Specie → Flora
│   └── instinct.rs    # Conversão Ethics → Instinct
└── eg/                 # Exemplos de arquivos SC
    └── sc.sc          # Exemplo base para testes

docs/                   # Documentação do projeto
├── architecture.md     # Visão geral e diagramas de arquitetura
├── usage.md            # Guia de uso da biblioteca
├── coding_standards.md # Padrões de codificação
└── contributing.md     # Guia de contribuição
```

## 🔧 Traits Principais

### `IRFromSC`
Trait responsável por converter uma árvore AST (`Tree`) em IR completo.

```rust
pub trait IRFromSC {
    fn from_sc(tree: Tree) -> IR;
}
```

### `LarvieFromBug`
Trait responsável por converter um `Bug` em `Larvie`.

```rust
pub trait LarvieFromBug {
    fn from_bug(bug: Bug) -> Larvie;
}
```

### `CastsFromGene`
Trait para conversão de `Gene` (definição de atributo) em `Casts` (elemento do IR).

```rust
pub trait CastsFromGene {
    fn from_gene(gene: Gene) -> Casts;
}
```

### `FloraFromSpecie`
Trait para conversão de tipos `Specie` em `Flora` (sistema de tipos do IR).

```rust
pub trait FloraFromSpecie {
    fn from_specie(specie: Specie) -> Flora;
}
```

### `InstinctFromEthics`
Trait para conversão de `Ethics` (comportamentos) em `Instinct` (instruções do IR).

```rust
pub trait InstinctFromEthics {
    fn from_ethics(ethics: Ethics) -> Instinct;
}
```

## 🚀 Uso

### Instalação

Adicione ao seu `Cargo.toml`:

```toml
[dependencies]
scir = { path = "/path/to/scir" }
ir = { path = "/path/to/ir" }
sc_dsl = { path = "/path/to/dsl" }
```

### Exemplo Básico

```rust
use scir::IRFromSC;
use ir::IR;
use sc_dsl::dsl::parser::tree::Tree;

// Converter código SC para IR
let sc_code = r#"bug Cat
  gene energy Int
  gene breath Int
  ethics move
end
"#;

let tree = Tree::parse_input(sc_code.to_string()).unwrap();
let ir = IR::from_sc(tree);

// O IR agora contém a representação intermediária otimizada
println!("IR gerado com {} alveolus", ir.alveolus.len());
```

### Exemplo de Conversão de Tipos

```rust
use scir::FloraFromSpecie;
use ir::Flora;
use sc_dsl::dsl::ast::emitter::Specie;

// Conversão manual de tipos
let specie = Specie { raw: "Int".to_string() };
let flora = Flora::from_specie(specie);
assert_eq!(flora, Flora::Int);
```

## 🧪 Testes

O projeto possui uma suíte abrangente de testes que validam:

- **Conversão completa**: Teste end-to-end da conversão SC → IR
- **Conversões de tipos**: Validação de mapeamento de tipos específicos
- **Integridade estrutural**: Verificação da estrutura do IR gerado

Para executar os testes:

```bash
cargo test
```

Para executar com output detalhado:

```bash
cargo test -- --nocapture
```

## 📦 Dependências

- **`ir`**: Sistema de representação intermediária - fornece tipos como `IR`, `Larvie`, `Casts`, `Flora`, `Instinct`
- **`sc_dsl`**: DSL para definição de smart contracts - fornece parser e tipos AST

## 🏗️ Arquitetura

### Conversões Implementadas

| Origem (SC DSL) | Destino (IR) | Status | Descrição |
|-----------------|--------------|--------|-----------|
| `Tree` | `IR` | ✅ | Conversão completa da árvore AST |
| `Bug` | `Larvie` | ✅ | Entidade principal do smart contract |
| `Gene` | `Casts` | ✅ | Atributos/propriedades |
| `Specie` | `Flora` | ✅ | Sistema de tipos |
| `Ethics` | `Instinct` | ✅ | Comportamentos/instruções |

### Fluxo de Conversão

```
SC Code → Tree (AST) → IR
    ↓         ↓        ↓
   "bug"    Bug    Larvie
   "gene"   Gene   Casts
   "ethics" Ethics Instinct
```

### Exemplo de Arquivo SC

```sc
bug Cat
  gene energy Int
  gene breath Int
  gene lives Int
  ethics move
  ethics hunt
end
```

### Resultado IR Correspondente

```rust
IR {
    alveolus: [
        Larvie {
            primor: Specie { raw: "Cat" },
            casts: [
                Casts { primor: "energy", flora: Flora::Int, seals: [Vital] },
                Casts { primor: "breath", flora: Flora::Int, seals: [Vital] },
                Casts { primor: "lives", flora: Flora::Int, seals: [Vital] },
            ],
            instincts: [
                Instinct { echo: "move" },
                Instinct { echo: "hunt" },
            ]
        }
    ]
}
```

## 📈 Performance

O SCIR foi projetado com foco em performance:

- **Zero-copy conversions**: Minimiza alocações desnecessárias
- **Lazy evaluation**: Conversões são feitas apenas quando necessário
- **Compilação otimizada**: Aproveitamento máximo das otimizações do Rust
- **Memory safety**: Garante segurança de memória sem overhead de runtime

## 🛠️ Desenvolvimento

### Configuração do Ambiente

```bash
# Clone o projeto
git clone <repository-url>
cd scir

# Executar testes
cargo test

# Executar com coverage
cargo test -- --nocapture

# Build de release
cargo build --release
```

### Adicionando Novas Conversões

Para adicionar uma nova conversão, siga este padrão:

1. **Defina o trait** em `src/traits/mod.rs`:
```rust
pub trait NewConversion {
    fn convert(input: InputType) -> OutputType;
}
```

2. **Implemente a conversão** em `src/extensions/`:
```rust
impl NewConversion for OutputType {
    fn convert(input: InputType) -> Self {
        // Lógica de conversão
    }
}
```

3. **Adicione testes** em `src/lib.rs`:
```rust
#[test]
fn test_new_conversion() {
    // Casos de teste
}
```

## 🔧 Estrutura Modular

A organização em módulos oferece:

1. **🎯 Separação de responsabilidades**: Cada módulo tem uma função específica
2. **🔌 Extensibilidade**: Fácil adição de novas conversões via traits
3. **🛠️ Manutenibilidade**: Código organizado e fácil de localizar
4. **✅ Testabilidade**: Cada módulo pode ser testado independentemente

### Design Patterns Utilizados

- **Trait Pattern**: Interface consistente para conversões
- **Extension Pattern**: Implementações externas para tipos de terceiros
- **Module Pattern**: Organização hierárquica do código

## 🎯 Status do Projeto

### ✅ Funcionalidades Implementadas

- [x] Trait `IRFromSC` para conversão completa
- [x] Trait `LarvieFromBug` para entidades principais
- [x] Trait `CastsFromGene` para atributos
- [x] Trait `FloraFromSpecie` para sistema de tipos
- [x] Trait `InstinctFromEthics` para comportamentos
- [x] Testes abrangentes com validação completa
- [x] Documentação e exemplos

### 🚀 Roadmap

- [ ] Suporte para múltiplos bugs por arquivo
- [ ] Conversões para `Behavior` (comportamentos complexos)
- [ ] Otimizações de performance
- [ ] Validação semântica adicional
- [ ] Geração de documentação automática
- [ ] Integração com ferramentas de análise estática
- [ ] Suporte para macros e metaprogramação

## 🤝 Contribuindo

Contribuições são bem-vindas! Por favor:

1. Fork o projeto
2. Crie uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

### Guidelines de Contribuição

- Mantenha o código bem documentado
- Adicione testes para novas funcionalidades
- Siga as convenções de código do Rust
- Atualize o README quando necessário

## 📋 Changelog

### v0.1.0 (2024-XX-XX)
- ✅ Implementação inicial do sistema de conversão
- ✅ Traits fundamentais para conversão SC → IR
- ✅ Testes abrangentes com validação completa
- ✅ Documentação e exemplos

## 📄 Licença

Este projeto está licenciado sob a licença especificada no arquivo `LICENSE`.
