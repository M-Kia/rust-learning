# Code Review for Lesson 2

## Task 1: Word Extractor
### Code:
```rust
fn first_word(sentence: &String) -> &str {
  let mut words = sentence.split(" ");
  let first_word = words.nth(0);
  match first_word {
    Some(word) => word,
    None => sentence,
  }
}
```

### Feedback:
1. **Correctness**:
   - The function correctly extracts the first word using `split(" ")` and `nth(0)`.
   - The `match` statement handles the case where no words are found.

2. **Improvement**:
   - Instead of returning the entire `sentence` in the `None` case, you should return an empty string (`""`) since the function is expected to return a `&str`.

### Suggested Fix:
```rust
fn first_word(sentence: &String) -> &str {
  let mut words = sentence.split(" ");
  let first_word = words.nth(0);
  match first_word {
    Some(word) => word,
    None => "",
  }
}
```

---

## Task 2: Ownership Transfer
### Code:
```rust
fn takes_ownership(s: String) {
  let _s1 = s;
}

fn makes_copy(n: i32) -> i32 {
  n.clone()
}
```

### Feedback:
1. **Correctness**:
   - `takes_ownership` correctly consumes the `String` by taking ownership.
   - `makes_copy` demonstrates that integers implement the `Copy` trait, so cloning is unnecessary.

2. **Improvement**:
   - The `clone()` call in `makes_copy` is redundant for `i32` since it is a `Copy` type. You can simply return `n`.

### Suggested Fix:
```rust
fn makes_copy(n: i32) -> i32 {
  n
}
```

---

## Task 3: Mutable Borrowing
### Code:
```rust
fn append_world(s: &mut String) {
  s.push_str(" world");
}
```

### Feedback:
- This function is perfect! It demonstrates mutable borrowing and modifies the string as expected.

---

## Main Function
### Code:
```rust
fn main() {
  // Task 1
  let sentence = String::from("Hello to all");
  let word = first_word(&sentence);
  
  println!("The first word of sentence is: {}", word);
  
  // Task 2
  let name = String::from("Mohammad-Hossein");
  takes_ownership(name);
  let age: i32 = 24;
  makes_copy(age);

  // Task 3
  let mut temp = String::from("Hello ");
  append_world(&mut temp);
  println!("The new sentence is: {}", temp);
}
```

### Feedback:
1. **Correctness**:
   - The `main` function demonstrates all tasks correctly.
   - The `println!` statements provide clear output for each task.

2. **Improvement**:
   - After calling `takes_ownership(name)`, the `name` variable is no longer valid. If you try to use it again, it will cause a compile-time error. This is fine here since you don't reuse `name`, but it's worth noting.