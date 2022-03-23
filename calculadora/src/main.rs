use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    // O primeiro argumento é a localização do binário compilado, então pule-o
    let first: String = args.nth(1).unwrap();
    // Depois de acessar o segundo argumento, o próximo elemento do iterador se torna o primeiro
    let operator: String = args.nth(0).unwrap();
    let second: String = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();

    println!("{} {} {}", first_number, operator, second_number);
}
