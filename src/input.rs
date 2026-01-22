use std::io::{self, Write}; // Write - добавляем для возможности использовать stdout()

/*
Функция обработкки ввода пользователя для возврата корректного 
числового значения в виде f64. 
На вход принимает &str - текст сообщающий о необходимости выполнения операции ввода
*/
pub fn get_input_number(prom: &str) -> Result<f64, String> {
    print!("{}", prom);
    let mut input_user = String::new();

    // stdout().flush() - Сброс буфера вывода
    io::stdout().flush().expect("Ошибка высвобождения буфера");
    io::stdin().read_line(&mut input_user).expect("Error input user...");
    
    // Создаем новую переменную типа Result и присваеваем ей input_user
    let input_number = input_user.trim().parse::<f64>();
    match input_number {
        Ok(number) => Ok(number),
        Err(err) => {
            /* 
                Тк input_user остался у нас строкой мы можем его вывести в ошибку!
                макрос format! делает тоже самое что и println!, но вместо вывода возращает сторку!
            */
            Err(format!("Ошибка ввода, {} не является числом! {}", input_user.trim(), err))
        }
    }
}

/*
Функкция обработки ввода пользорвателя для возврата оператора вычисления в виде char.
На вход принимает &str - текст сообщающий о необходимости выполнения операции ввода
*/
pub fn get_input_operator(prom: &str) -> Result<char, String> {
    print!("{}", prom);
    let mut input_user = String::new();
    io::stdout().flush().expect("Ошибка высвобождения буфера"); // Сброс буфера вывода
    io::stdin().read_line(&mut input_user).expect("Error input user...");
    let operator = input_user.trim().parse::<char>();
    match operator {
        Ok(oper) => Ok(oper),
        Err(err) => Err(format!("Ошибка ввода, {} не является знаком операции! {}", input_user.trim(), err))
    }
}