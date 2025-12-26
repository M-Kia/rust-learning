# Lesson 1: Rust Basics & Setup

## Key Concepts

### 1. Variables & Mutability
Unlike TypeScript/Python, Rust variables are **immutable by default**:

```rust
let x = 5;        // Immutable (like 'const' in TS)
// x = 6;         // ERROR!

let mut y = 5;    // Mutable (like 'let' in TS)
y = 6;            // OK!
```

**Why?** Rust prioritizes safety. Explicit mutability prevents bugs.

### 2. Type System
Rust is statically typed like TypeScript, but with type inference:

```rust
let x = 5;              // i32 inferred
let y: i64 = 5;         // Explicit type
let z: f64 = 3.14;      // Float
let name = "Alice";     // &str (string slice)
let active = true;      // bool
```

**Compare to TypeScript:**
```typescript
let x = 5;              // number inferred
let y: number = 5;      // Explicit
let name = "Alice";     // string
```

### 3. Functions
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = return value (like Ruby/Kotlin)
}

// Or explicit return:
fn subtract(a: i32, b: i32) -> i32 {
    return a - b;
}
```

**Compare to TypeScript:**
```typescript
function add(a: number, b: number): number {
    return a + b;
}
```

### 4. Control Flow
```rust
// If expressions (return values!)
let number = if condition { 5 } else { 6 };

// Loops
loop {
    // Infinite loop
    break;
}

while condition {
    // While loop
}

for i in 0..5 {  // Range: 0, 1, 2, 3, 4
    println!("{}", i);
}
```

### 5. Printing
```rust
println!("Hello, world!");           // Macro (note the '!')
println!("Value: {}", x);            // Like Python's format
println!("x: {}, y: {}", x, y);      // Multiple values
println!("Debug: {:?}", some_var);   // Debug print
```

## Key Differences from Your Languages

| Concept | TypeScript/Python | Rust |
|---------|------------------|------|
| Variables | Mutable by default | Immutable by default |
| Memory | Garbage collected | Owner-based (no GC!) |
| Null | `null`/`undefined`/`None` | No null! (Option<T>) |
| Errors | Exceptions | Result<T, E> |
| Strings | Simple | Complex (String vs &str) |

## Your First Exercise

Create a program in `exercise01.rs` that:

1. **Temperature Converter**
   - Create a function `celsius_to_fahrenheit(celsius: f64) -> f64`
   - Create a function `fahrenheit_to_celsius(fahrenheit: f64) -> f64`
   - In `main()`, test both functions with at least 3 different values
   - Print the results in a nice format

2. **Bonus Challenge**
   - Add a function `is_extreme_temp(celsius: f64) -> bool` that returns true if temperature is below -40°C or above 50°C
   - Use this in your main function

**Tips:**
- Formula: F = C × 9/5 + 32
- Formula: C = (F - 32) × 5/9
- Use `println!` for output
- All functions should be above `main()`

**To compile and run:**
```bash
rustc exercise01.rs
./exercise01
```

Or create it in `lesson01/exercise01.rs` and run:
```bash
cd lesson01
rustc exercise01.rs && ./exercise01
```

## Success Criteria
✓ Code compiles without warnings
✓ Functions work correctly
✓ Clean, readable code
✓ Good variable names

Once you're done, show me your code and I'll review it!
