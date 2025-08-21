fn main(){
    ownership();
    pattern_matching();
    erros();
}

fn ownership(){
    let mut minha_string = String::from("Lucas"); // aloca o texto na heap e minha_string aponta para esse texto
    rouba(&mut minha_string); // referencia mutavel ao valor da heap

    println!("{}", minha_string)
}

fn rouba(string: &mut String){ // recebe uma referencia mutavel
    println!("{}", string);
    string.push_str(" Phill"); // adiciona um valor ao texto na heap
}

fn pattern_matching(){
    for x in 1..=20 {
        println!("{}: {}", x, match x {
            1 => "Pouco",
            2 | 3 => "Um pouco",
            4..=10 => "Um cado",
            _ if x % 2 == 0 => "Um cadin a mais mas par",
            _ => "Muito"
        });
    }

    // match point {
    //     (0,0) => "Origem",
    //     (0,_) => "Eixo X",
    //     (_,_) => "Ué"
    // };
}

fn erros(){
    // panic!("Erro proposital");
    match resultado() {
        Ok(s) => println!("Sucesso string é: {}", s),
        Err(num) => println!("Codigo de erro: {}", num)
    }
}

fn resultado() -> Result<String, u8>{
    // Ok(String::from("Deu tudo certo"))
    Err(101)
}