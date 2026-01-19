mod calculator;
mod input;

use std::io;
use crate::{calculator::{Calculations, Calculator}, input::get_number};

fn main() {
    greetings();
    loop {
        let mut first_number  = String::new();
        let mut operation = String::new();
        let mut second_number = String::new();
        println!("Введите первое число (пример ввода - 0 или 0.0) или exit - для выхода:");
        io::stdin().read_line(&mut first_number).expect("Ошибка ввода пользоваттеля!");
        if first_number.contains("exit") {
            break println!("Спасибо! Пока!");
        }
        println!("Введите операцию (+, -, *, /):");
        io::stdin().read_line(&mut operation).expect("Ошибка ввода пользоваттеля!");
        println!("Введите второе число (пример ввода - 0 или 0.0):");
        io::stdin().read_line(&mut second_number).expect("Ошибка ввода пользоваттеля!");

        let first_number: f64 = first_number.trim().parse().expect("Error parse...");
        let operation: char = operation.trim().parse().expect("Error parse...");
        let second_number: f64 = second_number.trim().parse().expect("Error parse...");
        
        let calculator = Calculator { first_number, operation, second_number };
        match calculator.operation {
            '+' => calculation_result(Operation::SUM, calculator),
            '-' => calculation_result(Operation::DIFFERENCE, calculator),
            '*' => calculation_result(Operation::MULTIPLICATION, calculator),
            '/' => calculation_result(Operation::DIVISION, calculator),
            _ => println!("Неверный ввод операции. Введите: +, -, * или /")
        }
    }
}

enum Operation {
    SUM,
    DIFFERENCE,
    MULTIPLICATION,
    DIVISION
}

fn calculation_result(oper: Operation, result: Calculator) {
    match oper {
        Operation::SUM => println!("Результат: {} + {} = {}", result.first_number, result.second_number, result.sum()),
        Operation::DIFFERENCE => println!("Результат: {} - {} = {}", result.first_number, result.second_number, result.diff()),
        Operation::MULTIPLICATION => println!("Результат: {} * {} = {}", result.first_number, result.second_number, result.mult()),
        Operation::DIVISION => println!("Результат: {} / {} = {}", result.first_number, result.second_number, result.div())
    }
    println!();
    println!("+++++++++++++++++++++++++++");
    println!();
}

fn greetings() {
    println!("+++++++++++++++++++++++++++");
    println!("++++++  Калькулятор  ++++++");
    println!("+++++++++++++++++++++++++++");
    println!();
}