use std::env::{args, Args};

fn operate(operator: char, first_number: f32, second_numbers: f32) -> f32 {
    match operator {
        '+' => first_number + second_numbers,
        '-' => first_number - second_numbers,
        '/' => first_number / second_numbers,
        '*' | 'X' | 'x' => first_number * second_numbers,
        _ => panic!("Invalid operator used!"),
    }
}

fn main() {
    let mut args: Args = args();
    let first: String = args.nth(1).unwrap_or("".to_owned());
    let operator: String = args.nth(0).unwrap_or("".to_owned());
    let second: String = args.nth(0).unwrap_or("".to_owned());

    // parse numbers
    let first = first.parse::<f32>().unwrap();
    let second = second.parse::<f32>().unwrap();

    let res = operate(operator.parse::<char>().unwrap(), first, second);
    println!("{first} {operator} {second} = {res}")
}
