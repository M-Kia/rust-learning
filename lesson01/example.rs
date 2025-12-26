// Example: Basic Rust syntax demonstration

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // Variables
    let message = greet("Rustacean");
    println!("{}", message);
    
    // Numbers
    let width = 10.5;
    let height = 20.0;
    let area = calculate_area(width, height);
    println!("Area: {} (width: {}, height: {})", area, width, height);
    
    // Control flow
    let number = 42;
    if is_even(number) {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }
    
    // Loops
    println!("\nCounting:");
    for i in 1..=5 {  // 1 to 5 inclusive
        println!("  {}", i);
    }
    
    // Mutability
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 3 {
            break;
        }
        println!("Counter: {}", counter);
    }
}
