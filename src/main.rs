use std::io;

fn main() {
    greetings();
    
    get_data();
}

fn input_user() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Eror input user...");
    let input: f64 = input.trim().parse().expect("Error parse...");
    input
}

fn greetings() {
    println!("+++++++++++++++++++++++++++");
    println!("++++++  Калькулятор  ++++++");
    println!("+++++++++++++++++++++++++++");
}

fn get_data() {
    println!();
    println!("Введите первое число (пример ввода - 0 или 0.0):");
    let num_1 = input_user();
    println!("Введите второе число (пример ввода - 0 или 0.0):");
    let num_2 = input_user();

    println!("Сумма чисел равна: {}", num_1 + num_2);
}