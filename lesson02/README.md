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

### Simplified Explanation of Borrowing and Slices

#### Borrowing:
- Borrowing is like **lending** a value to a function without giving it away.
- **Immutable Borrowing**: You can read the value, but you can't change it.
  - Example: Lending a book to someone to read, but they can't write in it.
- **Mutable Borrowing**: You can change the value, but only one person can borrow it at a time.
  - Example: Lending a book to someone to edit, but only one person can edit it at a time.

#### Slices:
- A **slice** is like taking a **part** of a collection (e.g., a string or array) without copying it.
- Example: If you have a sentence, a slice can be just the first word or a specific part of it.
- Slices are references to the original data, so they don't own the data.

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

#### Step-by-Step Explanation of the `first_word` Function

```rust
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

#### What This Code Does:
1. **Input**: The function takes a reference to a `String` (`s: &String`).
   - Example: If `s = "hello world"`, the function will process this string.

2. **Convert to Bytes**:
   ```rust
   let bytes = s.as_bytes();
   ```
   - Converts the string into an array of bytes (`u8` values).
   - Example: `"hello world"` becomes `[104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]`.

3. **Iterate Over Bytes**:
   ```rust
   for (i, &item) in bytes.iter().enumerate() {
   ```
   - Loops through each byte in the array.
   - `enumerate()` gives both the index (`i`) and the value (`item`).

4. **Check for Space**:
   ```rust
   if item == b' ' {
       return &s[0..i];
   }
   ```
   - If the current byte is a space (`b' '`), return a slice of the string from the start (`0`) to the space's index (`i`).
   - Example: For `"hello world"`, the space is at index `5`, so it returns `"hello"`.

5. **Return the Whole String if No Space**:
   ```rust
   &s[..]
   ```
   - If no space is found, return the entire string as a slice.
   - Example: For `"hello"`, it returns `"hello"`.

### Example Walkthrough:

#### Input:
```rust
let s = String::from("hello world");
let word = first_word(&s);
println!("The first word is: {}", word);
```

#### Execution:
1. `s` is `"hello world"`.
2. `s.as_bytes()` converts it to `[104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]`.
3. The loop finds the space (`b' '`) at index `5`.
4. Returns the slice `&s[0..5]`, which is `"hello"`.
5. Prints: `The first word is: hello`.

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