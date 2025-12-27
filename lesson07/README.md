# Lesson 7: Lifetimes

## Key Concepts

### 1. What Are Lifetimes?
Lifetimes in Rust ensure that references are valid as long as they are used. They prevent dangling references and memory safety issues.

#### Example Without Lifetimes:
```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
This code will not compile because the compiler cannot determine if the returned reference will live long enough.

#### Example With Lifetimes:
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
- `'a` is a lifetime parameter that ensures the returned reference lives at least as long as both `x` and `y`.

### 2. Lifetime Annotations
Lifetime annotations specify the scope of a reference's validity. They do not change how long a reference lives but help the compiler understand relationships between lifetimes.

#### Syntax:
- `&'a T`: A reference with a lifetime `'a`.
- `<'a>`: Declares a lifetime parameter.

#### Example:
```rust
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

### 3. Structs with Lifetimes
When a struct contains references, you must annotate the lifetimes of those references.

#### Example:
```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Excerpt: {}", excerpt.part);
}
```

### 4. Lifetime Elision
In some cases, Rust can infer lifetimes, so you don’t need to annotate them explicitly.

#### Rules:
1. Each parameter gets its own lifetime.
2. If there is one input lifetime, it is assigned to the output.
3. If there are multiple input lifetimes, Rust cannot infer the output lifetime.

#### Example:
```rust
fn first_word(s: &str) -> &str {
    // Lifetime is inferred
    &s[..]
}
```

### 5. Static Lifetime
The `'static` lifetime means the reference lives for the entire duration of the program.

#### Example:
```rust
let s: &'static str = "I have a static lifetime.";
```

## Key Differences from Your Languages

| Concept         | TypeScript/Python | Rust                  |
|-----------------|-------------------|-----------------------|
| References      | Implicit          | Explicit lifetimes    |
| Memory Safety   | GC                | Ownership + Lifetimes |

## Your Exercise

### Task 1: Lifetime Annotations
1. Write a function `longest<'a>(x: &'a str, y: &'a str) -> &'a str` that returns the longest of two string slices.
2. Test this function with different string slices.

### Task 2: Structs with Lifetimes
1. Define a struct `BorrowedString<'a>` with a field `text: &'a str`.
2. Create an instance of this struct in `main()` and print its content.

### Task 3: Lifetime Elision
1. Write a function `first_char(s: &str) -> &str` that returns the first character of a string slice.
2. Use lifetime elision to simplify the function.

### Success Criteria
✓ Code compiles without warnings
✓ Functions work correctly
✓ Clean, readable code
✓ Good variable names

Once you're done, show me your code and I'll review it!