# Lesson 10: Testing & Documentation

## Key Concepts

### 1. Writing Tests
Rust has a built-in test framework. Write tests using the `#[test]` attribute.

#### Example:
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

### 2. Running Tests
Run tests with:
```bash
cargo test
```

### 3. Documentation
Use `///` comments to write documentation.

#### Example:
```rust
/// Adds two numbers.
///
/// # Examples
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Generate documentation with:
```bash
cargo doc --open
```

## Your Exercise

### Task 1: Write Tests
1. Write a function `multiply(a: i32, b: i32) -> i32`.
2. Write tests for this function.

### Task 2: Test Edge Cases
1. Write a function `divide(a: i32, b: i32) -> Option<i32>`.
2. Write tests for edge cases (e.g., division by zero).

### Task 3: Write Documentation
1. Document the `multiply` and `divide` functions.
2. Generate the documentation and verify it.

### Success Criteria
✓ Code compiles without warnings
✓ Functions work correctly
✓ Tests pass
✓ Documentation is clear and complete