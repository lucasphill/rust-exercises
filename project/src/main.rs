fn main() {
    const NAME: &str = "Lucas"; // Constantes nunca podem ser alteradas, nome da variavel em letra maiuscula. Precisa de ter seu tipo explicitado
    let my_num = 5;         // integer
    let my_double = 5.99;   // float
    let my_letter = 'D';    // character
    let my_bool = true;     // boolean
    let my_text = "Hello";  // string
    // let my_num: i32 = 5;          // integer
    // let my_double: f64 = 5.99;    // float
    // let my_letter: char = 'D';    // character
    // let my_bool: bool = true;     // boolean
    // let my_text: &str = "Hello";  // string
    println!("{}, world! My name is {} and I born in {}.", my_text, NAME, my_num);
    println!("Tenho {} na conta", my_double);
    println!("Uma letra: {}", my_letter);
    println!("Rust é legal? {}", my_bool);
}
