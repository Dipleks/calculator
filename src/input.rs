use std::io;
/*
Функция обработкки ввода пользователя для возврата корректного 
числового значения в виде f64. 
На вход принимает &str - текст сообщающий о необходимости выполнения операции ввода
*/
pub fn get_number(promt: &str) -> f64{
    println!("{}", promt);
    let mut input_user = String::new();
    io::stdin().read_line(&mut input_user).expect("Error input user...");
    
    // Обработать ошибку ввода, если значение не является числом
    let input_user = input_user.trim().parse().expect("Error parse input...");
    input_user
}

/*
Функкция обработки ввода пользорвателя для возврата оператора вычисления в виде char.
На вход принимает &str - текст сообщающий о необходимости выполнения операции ввода
*/
pub fn get_operator(promt: &str) -> char {
    '0'
}