# Alexandria

Uma API REST de **gerenciamento de processos do PROCON**. Futuramente, será consumida pela aplicação gráfica **[Pharos](https://github.com/mykaelAlves/Pharos)**.

## Funcionalidades

## Instalação

## Projeto

### Formatação

Execute ```cargo +nightly fmt``` para formatar o código de acordo com o **rustfmt.toml**.

### Arquitetura

O projeto utiliza uma divisão entre **services** (lógica de negócio), **handlers** (lidam com requisições, utilizam os services) e **models** (estruturas).
