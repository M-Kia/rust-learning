# Code Review for Lesson 5

## Task 1: Vector Operations
### Code:
```rust
let mut numbers = vec![];
numbers.push(2);
numbers.push(7);
numbers.push(8);
numbers.push(11);
numbers.push(13);
numbers.pop();
let sum: i32 = numbers.iter().sum();
println!("Sum of {:?} is {}", numbers, sum);
```

### Feedback:
1. **Correctness**:
   - The vector is correctly initialized and populated with integers.
   - The `pop` method removes the last number (`13`), and the sum of the remaining numbers is calculated correctly.

2. **Readability**:
   - The code is clean and easy to follow.

3. **Improvement**:
   - You could initialize the vector with values directly using `vec![...]` to make the code more concise.

### Suggested Improvement:
```rust
let mut numbers = vec![2, 7, 8, 11, 13];
numbers.pop();
let sum: i32 = numbers.iter().sum();
println!("Sum of {:?} is {}", numbers, sum);
```

---

## Task 2: HashMap Operations
### Code:
```rust
let mut scores = HashMap::new();
scores.insert("Mohammad-Hossein", 16);
scores.insert("Amir-Hossein", 17);
scores.insert("Sadegh", 18);
scores.insert("Mohammad-Hossein", 20);

for (name, score) in scores.drain() {
    println!("{}'s score is {}", name, score);
}
```

### Feedback:
1. **Correctness**:
   - The `HashMap` is correctly initialized, and entries are added.
   - The second `insert` for `"Mohammad-Hossein"` correctly updates the score to `20`.
   - The `drain` method is used to iterate over and remove all entries.

2. **Readability**:
   - The code is clear and demonstrates the use of `HashMap` effectively.

3. **Improvement**:
   - Consider using `iter` instead of `drain` if you don't need to remove the entries from the `HashMap`.

### Suggested Improvement:
```rust
for (name, score) in &scores {
    println!("{}'s score is {}", name, score);
}
```

---

## Task 3: Iterator Adapters
### Code:
```rust
let numbers: Vec<i32> = vec![2, 5, 7, 8, 11, 26];
let changed_numbers: Vec<i32> = numbers.into_iter().filter(|x| x % 2 == 0).map(|x| x.pow(2)).collect();

println!("New numbers are {:?}", changed_numbers);
```

### Feedback:
1. **Correctness**:
   - The `filter` method correctly keeps only even numbers.
   - The `map` method squares each even number.
   - The `collect` method gathers the results into a new vector.

2. **Readability**:
   - The code is concise and demonstrates the use of iterator adapters effectively.

3. **Improvement**:
   - None! This implementation is perfect.

---

## Overall Suggestions
1. **Consistency**:
   - Use consistent methods for iteration. For example, use `iter` for `HashMap` if you don't need to remove entries.

2. **Conciseness**:
   - Initialize vectors with values directly when possible.

3. **Edge Cases**:
   - Test additional edge cases, such as:
     - An empty vector for Task 1.
     - A `HashMap` with no entries for Task 2.
     - A vector with no even numbers for Task 3.