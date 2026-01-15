use std::io;

// Meu primeiro programa Rust
fn main() {
    let mut name: String = String::new();

    println!("Ola mundo! Meu nome é: Rogério Ramos Maldonado");
    println!("Informe um novo nome:");

    io::stdin().read_line(&mut name).expect("Failed to read line");

    println!("O novo nome é: {}", name);
}
