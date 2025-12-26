# Lesson 2: Ownership & Borrowing

## Key Concepts

### 1. Ownership
Ownership is Rust's most unique feature. It ensures memory safety without a garbage collector.

- **Rules of Ownership**:
  1. Each value in Rust has a single owner.
  2. When the owner goes out of scope, the value is dropped.
  3. You can transfer ownership (move) to another variable.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // Ownership moves to s2
    // println!("{}", s1); // ERROR: s1 is no longer valid
}
```

### 2. Borrowing
Borrowing allows you to use a value without taking ownership.

- **Immutable Borrowing**:
  - Multiple immutable borrows are allowed.

```rust
fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s); // Borrow s
    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but ownership is not taken
```

- **Mutable Borrowing**:
  - Only one mutable borrow is allowed at a time.

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

### 3. References & Slices
- **References**: Borrowed pointers to data.
- **Slices**: Borrowed views into a collection.

```rust
fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("The first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

## Key Differences from Your Languages

| Concept         | TypeScript/Python | Rust                  |
|-----------------|-------------------|-----------------------|
| Memory          | Garbage collected | Ownership model       |
| References      | Implicit          | Explicit borrowing    |
| Mutability      | Default mutable   | Default immutable     |

## Your Exercise

### Task 1: Word Extractor
Write a program in `exercise02.rs` that:

1. Takes a sentence as input.
2. Extracts the first word using a function `first_word(sentence: &String) -> &str`.
3. Prints the first word.

### Task 2: Ownership Transfer
1. Create a function `takes_ownership(s: String)` that consumes a string.
2. Create a function `makes_copy(n: i32)` that takes an integer (copy type).
3. Demonstrate ownership transfer in `main()`.

### Task 3: Mutable Borrowing
1. Create a function `append_world(s: &mut String)` that appends `" world"` to a string.
2. Call it in `main()` and print the result.

### Success Criteria
✓ Code compiles without warnings
✓ Functions work correctly
✓ Clean, readable code
✓ Good variable names

Once you're done, show me your code and I'll review it!