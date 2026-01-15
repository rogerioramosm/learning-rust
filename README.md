# Aprendendo Rust

Para criar um projeto rust com `cargo`

```bash
  # project é o nome da pasta que será criada com o cargo configurado
  cargo new project

  # para compilar e rodar o projeto
  cargo run

  # para somente compilar o projeto
  cargo build
```

* Para compilar um arquivo `.rs` específico

```bash

  # rustc -> compilar o arquivo `file.rs`
  # -o -> especifica o local de saída do arquivo criando o executável compilado
  rustc file.rs -o ./file.exe

```
