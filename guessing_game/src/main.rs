use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    
    loop{
        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("Entre com um numero: ");
        
        let mut guess = String::new(); // mut informa que a variavel pode receber novo valor durante a execução
        
        println!("O numero secreto é {secret_number}");

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Venceu!");
                break;
            },
        }   
        println!("Voce disse: {guess} e o numero era {secret_number}")
    }
    
}
