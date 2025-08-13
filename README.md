[https://doc.rust-lang.org](https://doc.rust-lang.org/stable/book/title-page.html)
### Declarando variavel
`let mut guess = String::new();`
- `let` - informa ao compilador que criará uma nova variável
- `mut` - informa que a variavel é "mutavel"
- `guess` - nome da variavel
- `String::new()` - informa que será uma instancia de uma nova string

#### Declarando uma constante
`const NAME: &str = "Lucas";`
- Constantes nunca podem ser alteradas
- nome da variavel em letra maiuscula.
- Precisa de ter seu tipo explicitado

#### Tipos de variaveis
- `let my_num: i32 = 5;` ou `let my_num = 5;`
- `let my_double: f64 = 5.99;`
- `let my_letter: char = 'D';`
- `let my_bool: bool = true;`
- `let my_text: &str = "Hello";`

### Printando valores
- `println!("Tenho {} na conta", my_double);`
- `println!("Voce disse: {guess} e o numero era {secret_number}")`

### Adicionando dependencias
```rust
use std::{cmp::Ordering, io};
use rand::Rng;
```

#### Instalando novas dependencias
```toml
// Cargo.toml
[dependencies]
rand = "0.8.5"
```
#### Atualizando dependencias
`cargo update`

### Recebendo valor pelo terminal
```rust
io::stdin()
.read_line(&mut guess)
.expect("Failed to read the line");
```
- `io` - biblioteca importada que possui a função
- `::stdin()` - função padrão da biblioteca io para obter um valor do terminal
- `.read_line(&mut guess)` - faz a leitura da entrada do usuário no terminal. &mut permite que a função altere o valor da variavel guess.
- `expect("mensagem")` - Trata possiveis erros na leitura. Se ocorrer erro, exibe a mensagem e encerra o programa. Se a leitura for bem sucedida o programa continua normalmente

### Convertendo valor texto para numero
```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```
- `let guess: u32` - declara uma variavel chamada guess, inteiro, sem sinal
- `guess.trim()` - remove espaços em branco do inicio e fim
- `.parse()` - tenta converter para o tipo especificado
- `match` - estrutura de controle que verifica o resultado da conversão:
    - `OK(num) => num` - se a conversão for ok retorna o numero convertido
    - `Err(_) => continue` - se houver erro ignora e continua

### Fazendo loop
```rust
loop{
    if 1 > 0 {
        break;
    }
}
```
- `loop{}` - Cria um loop infinito
- `break;` - encerra o loop, 

###
```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Muito baixo!"),
    Ordering::Greater => println!("Muito alto!"),
    Ordering::Equal => {
        println!("Venceu!");
        break; // encerrar loop
    },
}   
```
- `match guess.cmp(&secret_number)` - compara o valor de 'guess' usando o metodo cmp com o valor da variavel apontada em '&secret_number'
- `Ordering::Less` - Caso o valor de guess seja menor
- `Ordering::Greater` - Caso o valor de guess seja maior
- `Ordering::Equal` - Caso o valor de guess seja igual