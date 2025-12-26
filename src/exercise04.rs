use std::{error::Error, fmt, fs, io};

fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeLogarithm,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Error: Division by zero is not allowed!"),
            MathError::NegativeLogarithm => write!(f, "Error: Logarithm of a negative number is undefined!")
        }
    }
}

impl Error for MathError {}

fn divide(dividend: f64, divisor: f64) -> Result<f64, MathError> {
    if divisor != 0. {
        Ok(dividend / divisor)
    } else {
        Err(MathError::DivisionByZero)
    }
}

fn safe_log(x: f64) -> Result<f64, MathError> {
    if x < 0. {
        Err(MathError::NegativeLogarithm)
    } else {
        Ok(x.log10())
    }
}

fn main() {
    // Task 1
    let paths = ["./data.txt", "wrong-path.txt"];
    for path in paths {
        match read_file(path) {
            Ok(data) => println!("Content of {}:\n{}", path, data),
            Err(msg) => println!("Failed to read file '{}':\n{}", path, msg),
        };
    }

    // Task 2
    let numbers = [(40., 5.), (20., 0.)];
    for &(dividend, divisor) in &numbers {
        match divide(dividend, divisor) {
            Ok(result) => println!("{} divided by {} is {}", dividend, divisor, result),
            Err(message) => println!("{}", message),
        };
    }

    // Task 3
    let numbers = [-10., 10.];
    for &num in &numbers {
        match safe_log(num) {
            Ok(res) => println!("{} Log 10 is {}", num, res),
            Err(msg) => println!("{}", msg),
        }
    }
}
