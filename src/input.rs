use std::io::{self, Write}; // Write - добавляем для возможности использовать stdout()

/*
Функция обработкки ввода пользователя для возврата корректного 
числового значения в виде f64. 
На вход принимает &str - текст сообщающий о необходимости выполнения операции ввода
*/
pub fn get_number(promt: &str) -> f64 {
    print!("{}", promt);
    let mut input_user = String::new();
    io::stdout().flush().expect("Ошибка высвобождения буфера"); // Сброс буфера вывода
    io::stdin().read_line(&mut input_user).expect("Error input user...");

    // Обработать ошибку ввода, если значение не является числом
    let input_user: f64 = input_user.trim().parse().expect("Error parse input...");
    input_user
}

pub fn get_input_number(promt: &str) -> Result<f64, String> {
    print!("{}", promt);
    let mut input_user = String::new();
    io::stdout().flush().expect("Ошибка высвобождения буфера"); // Сброс буфера вывода
    io::stdin().read_line(&mut input_user).expect("Error input user...");
    
    input_user.trim().parse::<f64>().map_err(|_| format!("Error number {}", input_user.trim()))
}

/*
Функкция обработки ввода пользорвателя для возврата оператора вычисления в виде char.
На вход принимает &str - текст сообщающий о необходимости выполнения операции ввода
*/
pub fn get_operator(promt: &str) -> char {
    print!("{}", promt);
    let mut input_user = String::new();
    io::stdout().flush().expect("Ошибка высвобождения буфера"); // Сброс буфера вывода
    io::stdin().read_line(&mut input_user).expect("Error input user...");

    // Обработать ошибку ввода, если значение не является числом
    let input_user: char = input_user.trim().parse().expect("Error parse input...");
    input_user
}