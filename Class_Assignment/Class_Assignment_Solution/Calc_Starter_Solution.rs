// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}

fn main() {
    println!("Calculator!");
    println!("---------");

    loop {
        let mut value1 = String::new();
        let mut value2 = String::new();
        let mut operation = String::new();

        print!("Please enter your first number?: ");
        read(&mut value1);

        print!("Please enter your second number?: ");
        read(&mut value2);

        print!("what operation would you like to do? [+-*/]: ");
        read(&mut operation);

        let value1: f32 = value1.trim().parse().unwrap();
        let value2: f32 = value2.trim().parse().unwrap();
        let operation: char = operation.trim().chars().next().unwrap();

        let operations = String::from("+-*/");

        if !operations.contains(operation) {
            println!("Please review your response, operation entered is unknown.");
            continue;
        }

        let result = match operation {
            '+' => value1 + value2,
            '-' => value1 - value2,
            '*' => value1 * value2,
            '/' => value1 / value2,
            _ => panic!("error in operation")
        };

        println!("The result of {} {} {} = {}", value1, operation, value2, result);
    }
}