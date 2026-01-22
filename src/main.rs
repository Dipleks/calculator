mod calculator;
mod input;

use crate::{calculator::{Calculations, Calculator}, input::{get_input_number, get_operator}};

fn main() {
    greetings();
    loop {
        let prom_first = "Введите первое число: ";
        let prom_second = "Введите второе число: ";
        
        let first = match get_input_number(prom_first) {
            Ok(num) => num,
            Err(e) => {
                if e.contains("exit") {
                    break;
                }
                println!("{}", e);
                println!("Не корретный ввод! Введите число, пример ввода - 0 или 0.0");
                continue;
            }
        };
        let operation: char = get_operator("Введите операцию (+, -, *, /): ");
        
        let second = match get_input_number(prom_second) {
            Ok(num) => num,
            Err(e) => {
                if e.contains("exit") {
                    break;
                }
                println!("{}", e);
                println!("Не корретный ввод! Введите число, пример ввода - 0 или 0.0");
                continue;
            }
        };
        let calculator = Calculator { first_number: first, operation, second_number: second };
        calculator.calculate();
    }
}

fn greetings() {
    println!("+++++++++++++++++++++++++++");
    println!("++++++  Калькулятор  ++++++");
    println!("+++++++++++++++++++++++++++");
    println!();
}