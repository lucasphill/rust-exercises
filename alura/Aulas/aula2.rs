const PI: f32 = 3.14159265359;
static mut GLOBAL: i8 = 1;

fn soma (a:i32, b:i32) -> i32 {
    println!("{} + {} = {}", a, b, a + b);
    a + b
}


fn main() {
    println!("PI tem o valor de: {PI}");
    unsafe{
        println!("'global' tem o valor de: {GLOBAL}");
    }

    let variavel:i32 = 300;
    println!("Variavel tem o valor de: {variavel} e o tamanho de {} em bytes.", std::mem::size_of_val(&variavel));

    let decimal = 2.5;
    println!("Decimal tem o valor de: {decimal}");

    let booleano = false;
    // booleano = true;
    println!("Booleano tem o tamanho de {} bytes e o valor de {}", std::mem::size_of_val(&booleano), booleano);

    let character = 'C';
    println!("Character tem o tamanho de {} bytes", std::mem::size_of_val(&character));

    soma(22, 48);
}