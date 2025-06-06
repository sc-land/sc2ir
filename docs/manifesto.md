# SC2IR - Manifesto e Visão

## Visão

O SC2IR é uma ferramenta de transformação de código que visa simplificar o processo de desenvolvimento de contratos inteligentes, permitindo que os desenvolvedores escrevam código em uma linguagem de domínio específico (SC DSL) de alto nível e o convertam para uma representação intermediária (IR) otimizada.

## Objetivos

1. **Abstração**: Fornecer uma camada de abstração sobre a complexidade da representação intermediária.
2. **Segurança**: Garantir que transformações preservem a semântica do código original.
3. **Extensibilidade**: Criar uma arquitetura que permita fácil extensão para novos recursos.
4. **Composabilidade**: Permitir que componentes sejam combinados de maneira modular.
5. **Desempenho**: Otimizar o código gerado para execução eficiente.

## Princípios de Design

1. **Trait-Based**: Usar traits para definir interfaces de conversão claras.
2. **Testabilidade**: Todo código deve ser bem testado e verificável.
3. **Documentação Completa**: Manter documentação clara e abrangente.
4. **Sem Estado Global**: Evitar estado global para facilitar testes e paralelismo.
5. **Verificação Estrita**: Validar a integridade das conversões com verificações rigorosas.

## Roadmap

### Curto Prazo
- Suporte para todos os tipos básicos do SC DSL
- Conversão completa de bugs e suas propriedades
- Testes abrangentes para garantir a correção

### Médio Prazo
- Suporte para comportamentos complexos
- Otimizações de conversão
- Ferramentas de análise e validação

### Longo Prazo
- Suporte para metaprogramação
- Integração com ferramentas de desenvolvimento
- Ecossistema completo para desenvolvimento de contratos
