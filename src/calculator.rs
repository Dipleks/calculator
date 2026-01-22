use crate::input::{get_input_number, get_input_operator};

/*
    Струкрута Калькулятор, включает два числа и знак 
    выполняемой операции для вычисления.
*/
pub struct Calculator {
    pub first_number: f64,
    pub operation: char,
    pub second_number: f64,
}

// Трейт Вычисления, содержит функции для необходимых вычислений
pub trait Calculations {
    fn sum(&self) -> f64; // сумма
    fn diff(&self) -> f64; // разность
    fn mult(&self) -> f64; // умножение
    fn div(&self) -> f64; // деление
    fn calculate(&self);
}

// Реализация структуры Калькулятор
impl Calculator {
    // Создает новый Калькулятор с заданными параметрами для вычислений
    pub fn new(first_number: f64, operation: char, second_number: f64) -> Self {
        Self { first_number, operation, second_number }
    }
}

impl Calculations for Calculator {    
    fn sum(&self) -> f64 {
        self.first_number + self.second_number
    }

    fn diff(&self) -> f64 {
        self.first_number - self.second_number
    }

    fn mult(&self) -> f64 {
        self.first_number * self.second_number
    }

    fn div(&self) -> f64 {
        self.first_number / self.second_number
    }

    // TODO Необходима обработка деления на ноль
    fn calculate(&self) {
        match self.operation {
            '+' => println!("Результат: {} + {} = {}", self.first_number, self.second_number, self.sum()),
            '-' => println!("Результат: {} - {} = {}", self.first_number, self.second_number, self.diff()),
            '*' => println!("Результат: {} * {} = {}", self.first_number, self.second_number, self.mult()),
            '/' => println!("Результат: {} / {} = {}", self.first_number, self.second_number, self.div()),
            _ => println!("Неверный ввод операции. Введите: +, -, * или /")
        }
    }
}

pub fn process_calculation() {
    loop {
        let prom_first = "Введите первое число: ";
        let prom_operator = "Введите операцию (+, -, *, /): ";
        let prom_second = "Введите второе число: ";
        
        let first = match get_input_number(prom_first) {
            Ok(num) => num,
            Err(message) => {
                if message.contains("exit") {
                    break println!("Завершение работы...");
                }
                println!("{}", message);
                println!("Не корретный ввод! Введите число, пример ввода - 0 или 0.0");
                continue;
            }
        };
        let operator: char = match get_input_operator(prom_operator) {
            Ok(ch) => ch,
            Err(message) => {
                if message.contains("exit") {
                    break println!("Завершение работы...");
                }
                println!("{}", message);
                println!("Не корретный ввод! Введите операцию, пример ввода - +, -, *, /");
                continue;
            }
        };
        
        let second = match get_input_number(prom_second) {
            Ok(num) => num,
            Err(message) => {
                if message.contains("exit") {
                    break println!("Завершение работы...");
                }
                println!("{}", message);
                println!("Не корретный ввод! Введите число, пример ввода - 0 или 0.0");
                continue;
            }
        };
        let calculator = Calculator::new(first, operator, second);
        calculator.calculate();
    }
}