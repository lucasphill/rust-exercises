
fn soma (a:i32, b:i32) -> i32 {
    println!("{} + {} = {}", a, b, a + b);
    a + b
}


fn condicionais(idade:u32, responsavel:bool) {
    let maior = idade >= 18;

    if maior {
        println!("Pode entrar");
    }
    else if idade >= 16 && responsavel{
        println!("Pode entrar com autorização do responsavel");
    }
    else {
        println!("Não pode entrar");
    }

    let condicao = if maior { "Maior" } else { "Menor" };

    println!("{} de idade", condicao);

    let linguagem = "Terraform";
    let proposito = match linguagem {
        "PHP" => "Web",
        "Swift" => "iOS",
        "Golang" => "Infraestrutura",
        "C#" => "Windows",
        _ => "Desconhecido"
    };

    println!("Linguagem: {}, Proposito: {}", linguagem, proposito);
}

fn main() {
    soma(22, 48);

    condicionais(17, true);
    repeticoes(5);
}

fn repeticoes(multiplicador: u8){
    let mut contador:u8 = 0;
    while contador < 10{
        contador += 1;

        if contador == 3 {
            continue;
        }

        println!("{} X {} = {}", multiplicador, contador, multiplicador * contador);
    }
    
    contador = 0;
    loop {
        contador += 1;
        
        println!("{} X {} = {}", multiplicador, contador, multiplicador * contador);

        if contador == 10 {
            break; 
        }
    }

    for i in 1..11 { // ou for i in 1..=10 {
        println!("{} X {} = {}", multiplicador, i, multiplicador * i);
    }
}