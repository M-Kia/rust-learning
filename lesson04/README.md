# Lesson 4: Error Handling

## Key Concepts

### 1. The `Result` Enum
Rust uses the `Result` enum for error handling. It has two variants:
- `Ok(T)` - Represents a successful operation.
- `Err(E)` - Represents an error.

#### Example:
```rust
fn divide(dividend: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    match divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
```

### 2. Propagating Errors
Use the `?` operator to propagate errors. It returns the error to the caller if the `Result` is `Err`.

#### Example:
```rust
fn read_file(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?; // Propagates error if any
    Ok(content)
}
```

### 3. The `Option` Enum
Use `Option` when a value might be absent. It has two variants:
- `Some(T)` - Represents a value.
- `None` - Represents the absence of a value.

#### Example:
```rust
fn find_word(sentence: &str, word: &str) -> Option<usize> {
    sentence.find(word)
}

fn main() {
    match find_word("hello world", "world") {
        Some(index) => println!("Found at index: {}", index),
        None => println!("Not found"),
    }
}
```

### 4. Custom Errors
You can define your own error types by implementing the `std::fmt::Display` and `std::error::Error` traits.

#### Example:
```rust
use std::fmt;

#[derive(Debug)]
struct CustomError;

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Custom error occurred")
    }
}

impl std::error::Error for CustomError {}
```

### 5. Working with Files
Rust provides the `std::fs` module for file operations. Here are some common tasks:

#### Reading a File
Use `std::fs::read_to_string` to read the entire content of a file into a `String`:
```rust
use std::fs;
use std::io;

fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn main() {
    match read_file("example.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
}
```

#### Writing to a File
Use `std::fs::write` to write a `String` or `&[u8]` to a file:
```rust
use std::fs;
use std::io;

fn write_file(path: &str, content: &str) -> Result<(), io::Error> {
    fs::write(path, content)
}

fn main() {
    match write_file("example.txt", "Hello, world!") {
        Ok(_) => println!("File written successfully"),
        Err(e) => println!("Error writing file: {}", e),
    }
}
```

#### Appending to a File
Use `std::fs::OpenOptions` to append to a file:
```rust
use std::fs::OpenOptions;
use std::io::Write;

fn append_to_file(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)?;
    writeln!(file, "{}", content)
}

fn main() {
    match append_to_file("example.txt", "Appended content") {
        Ok(_) => println!("Content appended successfully"),
        Err(e) => println!("Error appending to file: {}", e),
    }
}
```

#### Creating a File
Use `std::fs::File` to create a new file:
```rust
use std::fs::File;
use std::io;

fn create_file(path: &str) -> Result<(), io::Error> {
    File::create(path)?;
    Ok(())
}

fn main() {
    match create_file("new_file.txt") {
        Ok(_) => println!("File created successfully"),
        Err(e) => println!("Error creating file: {}", e),
    }
}
```

#### Deleting a File
Use `std::fs::remove_file` to delete a file:
```rust
use std::fs;
use std::io;

fn delete_file(path: &str) -> Result<(), io::Error> {
    fs::remove_file(path)
}

fn main() {
    match delete_file("example.txt") {
        Ok(_) => println!("File deleted successfully"),
        Err(e) => println!("Error deleting file: {}", e),
    }
}
```

## Key Differences from Your Languages

| Concept         | TypeScript/Python | Rust                  |
|-----------------|-------------------|-----------------------|
| Exceptions      | `try`/`catch`    | `Result` and `Option` |
| Null            | `null`/`None`    | `Option<T>`           |
| Error Propagation| Implicit         | Explicit with `?`     |

## Your Exercise

### Task 1: File Reader
1. Write a function `read_file(path: &str) -> Result<String, std::io::Error>` that reads a file and returns its content.
2. In `main()`, call this function and handle errors using `match`.

### Task 2: Division with Error Handling
1. Modify the `divide` function from Lesson 3 to return a `Result<f64, String>`.
2. Use `match` in `main()` to handle the result.

### Task 3: Custom Error
1. Define a custom error type `MathError` with variants:
   - `DivisionByZero`
   - `NegativeLogarithm`
2. Write a function `safe_log(x: f64) -> Result<f64, MathError>` that:
   - Returns `Err(MathError::NegativeLogarithm)` if `x < 0.0`.
   - Otherwise, returns `Ok(x.log10())`.
3. Test this function in `main()`.

### Success Criteria
✓ Code compiles without warnings
✓ Functions work correctly
✓ Clean, readable code
✓ Good variable names

Once you're done, show me your code and I'll review it!