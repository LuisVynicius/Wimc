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