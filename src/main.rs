mod calculator;
mod input;

use crate::{calculator::{process_calculation}};

fn main() {
    greetings();
    process_calculation();
}

fn greetings() {
    println!("+++++++++++++++++++++++++++");
    println!("++++++  Калькулятор  ++++++");
    println!("+++++++++++++++++++++++++++");
    println!();
}