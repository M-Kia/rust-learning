# Lesson 9: Modules & Crates

## Key Concepts

### 1. Modules
Modules help organize code into logical units.

#### Defining a Module:
```rust
mod greetings {
    pub fn hello() {
        println!("Hello!");
    }
}
```

#### Using a Module:
```rust
fn main() {
    greetings::hello();
}
```

### 2. Crates
Crates are Rust’s compilation units. A crate can be a library or a binary.

#### Creating a Library Crate:
```bash
cargo new my_library --lib
```

#### Using External Crates:
Add the crate to `Cargo.toml`:
```toml
[dependencies]
rand = "0.8"
```

Use the crate in your code:
```rust
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..101);
    println!("Random number: {}", random_number);
}
```

## Your Exercise

### Task 1: Create a Module
1. Create a module `math` with functions `add`, `subtract`, `multiply`, and `divide`.
2. Use this module in `main()`.

### Task 2: Create a Library Crate
1. Create a library crate `utils`.
2. Add a function `greet(name: &str)` to the library.
3. Use this library in a binary crate.

### Task 3: Use an External Crate
1. Add the `rand` crate to your project.
2. Write a program that generates 5 random numbers and prints them.

### Success Criteria
✓ Code compiles without warnings
✓ Functions work correctly
✓ Clean, readable code
✓ Good variable names