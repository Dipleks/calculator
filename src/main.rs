mod calculator;
mod input;

use crate::{calculator::{Calculations, Calculator}, input::{get_number, get_operator}};

fn main() {
    greetings();
    loop {
        let first_number: f64 = get_number("Введите первое число (пример ввода - 0 или 0.0) или exit - для выхода: ");
        let operation: char = get_operator("Введите операцию (+, -, *, /): ");
        let second_number: f64 = get_number("Введите второе число (пример ввода - 0 или 0.0): ");
        
        let calculator = Calculator { first_number, operation, second_number };
        calculator.calculate();
    }
}

fn greetings() {
    println!("+++++++++++++++++++++++++++");
    println!("++++++  Калькулятор  ++++++");
    println!("+++++++++++++++++++++++++++");
    println!();
}