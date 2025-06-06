# Exemplo de Código SC

Este diretório contém exemplos de código SC usado para testes e demonstrações.

## sc.sc

Este arquivo contém um exemplo básico de código SC:

```sc
bug Cat
  gene energy Int     # Gene "energy" do tipo Int
  gene breath Int     # Gene "breath" do tipo Int
  ethics move         # Comportamento "move"
end
```

### Explicação:

1. `bug Cat` - Define uma entidade chamada "Cat"
2. `gene energy Int` - Define um atributo chamado "energy" do tipo "Int"
3. `gene breath Int` - Define um atributo chamado "breath" do tipo "Int"
4. `ethics move` - Define um comportamento chamado "move"
5. `end` - Fecha a definição do bug

Este exemplo é usado nos testes unitários para verificar se o sistema de conversão SC2IR funciona corretamente.

Quando convertido para IR, este código produz:
- Um objeto `IR` contendo um `Alveolus::Larvie`
- O `Larvie` tem `primor` igual a "Cat"
- O `Larvie` contém dois `Casts`: "energy" e "breath", ambos com `Flora::Int`
- O `Larvie` contém um `Instinct` com `echo` igual a "move"
