pub struct Calculator {
    pub first_number: f64,
    pub operation: char,
    pub second_number: f64,
}


pub trait Calculations {
    fn sum(&self) -> f64; // сумма
    fn diff(&self) -> f64; // разность
    fn mult(&self) -> f64; // умножение
    fn div(&self) -> f64; // деление
    fn calculate(&self);
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