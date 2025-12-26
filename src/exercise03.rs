struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn call(msg: &Message) {
    match msg {
        Message::Quit => println!("Quit the game."),
        Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
        Message::Write(text) => println!("{}", text),
        Message::ChangeColor(r, g, b) => println!("Changing color to ({}, {}, {})", r, g, b),
    };
}

fn divide(dividend: f64, divisor: f64) -> Option<f64> {
    if divisor == 0. {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn main() {
    // Task 1
    let rectangle = Rectangle {
        width: 20,
        height: 10,
    };
    let rectangle_area = rectangle.area();
    println!("The area of current rectangle is: {}", rectangle_area);

    // Task 2
    call(&Message::Quit);
    call(&Message::Move { x: 32, y: 60 });
    call(&Message::Write(String::from("Hello Everyone!")));
    call(&Message::ChangeColor(256, 0, 0));

    // Task 3
    let numbers = [(20., 0.), (40., 5.)];
    for &(dividend, divisor) in &numbers {
        let result = divide(dividend, divisor);
        match result {
            Some(res) => println!("{} divided by {} is {}", dividend, divisor, res),
            None => println!("{} can not divide by zero!", dividend),
        };
    }
}
