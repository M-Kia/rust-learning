# Lesson 8: Generics & Advanced Types

## Key Concepts

### 1. Generics
Generics allow you to write flexible and reusable code for different types.

#### Example:
```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```
- `T` is a generic type parameter.
- `T: PartialOrd` ensures that `T` can be compared.

### 2. Advanced Types
#### Newtype Pattern
Use a tuple struct to create a new type.
```rust
struct Millimeters(u32);
struct Meters(u32);
```

#### Type Aliases
Create type aliases for complex types.
```rust
type Thunk = Box<dyn Fn() + Send + 'static>;
```

#### Never Type
The `!` type represents a value that never returns.
```rust
fn never_returns() -> ! {
    panic!("This function never returns!");
}
```

## Your Exercise

### Task 1: Generic Functions
1. Write a generic function `smallest<T: PartialOrd>(list: &[T]) -> &T` that returns the smallest element in a list.
2. Test this function with integers and floats.

### Task 2: Newtype Pattern
1. Define a tuple struct `Kilometers(f64)`.
2. Write a function that converts `Kilometers` to miles.

### Task 3: Type Aliases
1. Create a type alias `Callback` for `Box<dyn Fn() + Send + 'static>`.
2. Write a function that accepts a `Callback` and executes it.

### Success Criteria
✓ Code compiles without warnings
✓ Functions work correctly
✓ Clean, readable code
✓ Good variable names