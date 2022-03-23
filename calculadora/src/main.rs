use std::env::{args, Args};

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'X' | 'x' => first_number * second_number,
        _ => panic!("Invalid operator used."),
    }
}

fn main() {
    let mut args: Args = args();

    // O primeiro argumento é a localização do binário compilado, então pule-o
    let first: String = args.nth(1).unwrap();
    // Depois de acessar o segundo argumento, o próximo elemento do iterador se torna o primeiro
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second: String = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let result = operate(operator, first_number, second_number);

    println!("{} {} {}", first_number, operator, second_number);
}
