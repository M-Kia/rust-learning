# Lesson 3: Structs & Enums

## Key Concepts

### 1. Structs
Structs are custom data types that let you group related data together.

#### Defining a Struct:
```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

#### Creating an Instance:
```rust
fn main() {
    let user1 = User {
        username: String::from("john_doe"),
        email: String::from("john@example.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("Username: {}", user1.username);
}
```

#### Updating Structs:
```rust
let user2 = User {
    email: String::from("jane@example.com"),
    ..user1 // Copy remaining fields from user1
};
```

### 2. Tuple Structs
Structs without named fields:
```rust
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);
```

### 3. Enums
Enums allow you to define a type by enumerating its possible values.

#### Defining an Enum:
```rust
enum IpAddrKind {
    V4,
    V6,
}
```

#### Using Enums:
```rust
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}
```

#### Enums with Data:
```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

### 4. The `Option` Enum
Rust has no `null`. Instead, it uses `Option<T>` to represent a value that can be present or absent.

```rust
let some_number = Some(5);
let absent_number: Option<i32> = None;
```

## Key Differences from Your Languages

| Concept         | TypeScript/Python | Rust                  |
|-----------------|-------------------|-----------------------|
| Classes         | Yes               | No (use structs)      |
| Null            | `null`/`None`    | `Option<T>`           |
| Pattern Matching| Limited           | Powerful with `match` |

## Your Exercise

### Task 1: Define a Struct
1. Create a struct `Rectangle` with fields `width` and `height`.
2. Implement a function `area(rect: &Rectangle) -> u32` that calculates the area.
3. In `main()`, create a `Rectangle` and print its area.

### Task 2: Use Enums
1. Define an enum `Message` with variants:
   - `Quit`
   - `Move { x: i32, y: i32 }`
   - `Write(String)`
   - `ChangeColor(i32, i32, i32)`
2. Write a function `call(msg: &Message)` that:
   - Prints a message for `Quit`.
   - Prints the coordinates for `Move`.
   - Prints the string for `Write`.
   - Prints the RGB values for `ChangeColor`.
3. Test all variants in `main()`.

### Task 3: Option Enum
1. Write a function `divide(dividend: f64, divisor: f64) -> Option<f64>` that:
   - Returns `None` if the divisor is `0.0`.
   - Otherwise, returns `Some(result)`.
2. Use `match` in `main()` to handle the result of `divide`.

### Success Criteria
✓ Code compiles without warnings
✓ Functions work correctly
✓ Clean, readable code
✓ Good variable names

Once you're done, show me your code and I'll review it!