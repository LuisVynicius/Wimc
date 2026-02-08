# Wimc (Where Is My Command)

## Sobre

Wimc foi criado inicialmente para localizar prints de depuração esquecidos ao longo do código, mas evoluiu para uma ferramenta capaz de buscar qualquer comando ou palavra dentro de um ou mais projetos. Ele percorre diretórios de forma eficiente, oferecendo uma maneira prática de inspecionar e analisar grandes bases de código.

## Instalação
```bash
# Compile o projeto no modo de produção (rápido e enxuto)
cargo build --release

# O executável será criado em:
target/release/wimc # (.exe no Windows)

# No Linux:
# mova o executável para o binário do sistema com o comando
sudo cp target/release/wimc /usr/bin

# No windows:
# Basta mover o executável para um diretório presente no PATH do sistema
# (ex: C:\Windows\System32 ou outro de sua preferência).
```

## Como usar
```bash
# Na raiz do seu projeto, execute:
wimc <caminho> <comando> -<argumentos>

# Exemplo:
wimc . println! -de

# Argumentos extras.
-d — percorre todos os diretórios
-e — percorre todos os arquivos

# Arquivos gerados
# Após a execução, o Wimc cria dois arquivos de texto na raiz onde o comando foi executado:

wimc_results.txt — Lista todos os arquivos que correspondem à busca realizada.
wimc_errors.txt — Registra os arquivos que não puderam ser lidos ou apresentaram erro durante a análise.
```

## Resultados

### Wimc_results.txt

Esse arquivo conterá um resumo dos resultados na primeira linha, seguindo pelos caminhos e as linhas correspondentes ao comando solicitado.

```
Total_files: 5 | Total_lines: 16

Path: ./README.pt_br.md
Lines: [48, 51, 54]

Path: ./src/args.rs
Lines: [1, 12, 19]

Path: ./src/commands.rs
Lines: [3, 11, 30]

Path: ./src/entity.rs
Lines: [1, 4, 10, 27]

Path: ./src/file.rs
Lines: [4, 12, 63]
```

### Wimc_errors.txt

Esse arquivo conterá um resumo dos erros na primeira linha, seguindo pelos caminhos e o tipo de erro ao tentar ler o arquivo.

```
Total_files: 139

Path: ./.git/index
Error: stream did not contain valid UTF-8

Path: ./.git/objects/09/ff8c2a84cf0ad520dcb4322b537e562a2a25b2
Error: stream did not contain valid UTF-8

Path: ./.git/objects/0b/2666c1a7673817b2c6d1d4e6e1f8efc38b1e4a
Error: stream did not contain valid UTF-8

Path: ./.git/objects/0c/c4b75e4e985d5bedf7afc0303eddeb347371ca
Error: stream did not contain valid UTF-8
```

## Dicas

Sem o uso do argumento -d, diretórios como /target são ignorados, mas caso o wimc seja executado dentro do diretório, ele funcionará normalmente.

## Personalização
Para adicionar novos diretórios ou extensões a lista de ignorados, abra o arquivo ignore.rs e adicionar os itens desejados a ignored_files(diretórios) e ignored_extensions(extensões).

```rust
// Exemplo:
pub fn ignored_files() -> Vec<&'static str> {
    vec![
        "/.git",
        "/target", // Rust

        // Adicione novos caminhos aqui
    ]
}
```