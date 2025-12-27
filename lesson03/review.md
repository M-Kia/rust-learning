# Code Review for Lesson 3

## Task 1: Structs
### Code:
```rust
struct Rectangle {
  width: u32,
  height: u32
}

fn area(rect: &Rectangle) -> u32 {
  rect.width * rect.height
}
```

### Feedback:
1. **Correctness**:
   - The `Rectangle` struct is well-defined with `width` and `height` fields.
   - The `area` function correctly calculates the area using the struct fields.

2. **Readability**:
   - The code is clean and easy to understand.

3. **Improvement**:
   - You could implement a method for the `Rectangle` struct to calculate the area, making the code more idiomatic.

### Suggested Improvement:
```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```
This allows you to call `rectangle.area()` directly.

---

## Task 2: Enums
### Code:
```rust
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32)
}

fn call(msg: &Message) {
  match msg {
      Message::Quit => println!("Quit the game."),
      Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
      Message::Write(text) => println!("{}", text),
      Message::ChangeColor(r, g, b) => println!("Changing color to ({}, {}, {})", r, g, b)
  };
}
```

### Feedback:
1. **Correctness**:
   - The `Message` enum is well-defined with all required variants.
   - The `call` function handles each variant correctly using `match`.

2. **Readability**:
   - The `match` arms are clear and concise.

3. **Improvement**:
   - None! This is a great implementation.

---

## Task 3: Option Enum
### Code:
```rust
fn divide(dividend: f64, divisor: f64) -> Option<f64> {
  match divisor {
      0. => None,
      _ => Some(dividend / divisor)
  }
}
```

### Feedback:
1. **Correctness**:
   - The `divide` function correctly returns `None` for division by zero and `Some(result)` otherwise.

2. **Readability**:
   - The `match` statement is clear and idiomatic.

3. **Improvement**:
   - You could simplify the `match` statement using an `if` condition:
     ```rust
     if divisor == 0.0 {
         None
     } else {
         Some(dividend / divisor)
     }
     ```

---

## Main Function
### Code:
```rust
fn main() {
  // Task 1
  let rectangle = Rectangle {
    width: 20,
    height: 10
  };
  let rectangle_area = area(&rectangle);
  println!("The area of current rectangle is: {}", rectangle_area);

  // Task 2
  call(&Message::Quit);
  call(&Message::Move { x: 32, y: 60 });
  call(&Message::Write(String::from("Hello Everyone!")));
  call(&Message::ChangeColor(256, 0, 0));

  // Task 3
  let numbers = [
    (20., 0.),
    (40., 5.)
  ];
  for (_i, value) in numbers.iter().enumerate() {
    let result = divide(value.0, value.1);
    match result {
        Some(res) => println!("{} divided by {} is {}", value.0, value.1, res),
        None => println!("{} can not divide by zero!", value.0)
    };
  };
}
```

### Feedback:
1. **Correctness**:
   - The `main` function demonstrates all tasks correctly.
   - The `println!` statements provide clear output for each task.

2. **Readability**:
   - The code is well-structured and easy to follow.

3. **Improvement**:
   - The `for` loop in Task 3 could use destructuring for clarity:
     ```rust
     for &(dividend, divisor) in &numbers {
         let result = divide(dividend, divisor);
         match result {
             Some(res) => println!("{} divided by {} is {}", dividend, divisor, res),
             None => println!("{} can not divide by zero!", dividend)
         };
     }
     ```