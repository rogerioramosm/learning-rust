use std::{ cmp::Ordering, io };

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::rng().random_range(1..=100);

    println!("The secret number is: {secret_number}");

    //Repete todo o escopo enquanto verdadeiro ou enquanto não executa exceção
    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        // Recebe valor informado pelo usuário
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // Variável inteira recebe a variável string informada acima
        // Faz validação e conversão do valor para inteiro
        // Se a conversão identificar que valor não é inteiro,
        // força reinserção do valor até que seja informado valor
        // inteiro esperado dentro do loop {}
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        println!("You guessed: {guess}");

        // Faz comparação do número informado e convertido para inteiro
        // com o número gerado randomicamente
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!!"),
            Ordering::Greater => println!("Too big!!!"),
            Ordering::Equal => {
                println!("You win!!!");
                break;
            }
        }
    }
}
