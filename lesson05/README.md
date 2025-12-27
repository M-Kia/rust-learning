# Lesson 5: Collections & Iterators

## Key Concepts

### 1. Collections
Rust provides powerful collection types in the `std::collections` module. Here are the most commonly used ones:

#### Vectors
A `Vec<T>` is a growable array.
```rust
fn main() {
    let mut numbers = vec![1, 2, 3];
    numbers.push(4);
    println!("{:?}", numbers); // [1, 2, 3, 4]
}
```

#### HashMaps
A `HashMap<K, V>` stores key-value pairs.
```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 10);
    scores.insert("Bob", 20);
    println!("{:?}", scores);
}
```

#### HashSets
A `HashSet<T>` stores unique values.
```rust
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(2); // Duplicate, won't be added
    println!("{:?}", set); // {1, 2}
}
```

### 2. Iterators
Iterators allow you to process a sequence of items lazily.

#### Creating an Iterator
```rust
fn main() {
    let numbers = vec![1, 2, 3];
    let mut iter = numbers.iter();
    println!("{:?}", iter.next()); // Some(1)
    println!("{:?}", iter.next()); // Some(2)
}
```

#### Using Iterator Adapters
- **`map`**: Transform each item.
- **`filter`**: Keep items that satisfy a condition.
- **`collect`**: Consume the iterator and collect results into a collection.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];
    let squares: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    let evens: Vec<i32> = numbers.into_iter().filter(|x| x % 2 == 0).collect();
    println!("Squares: {:?}", squares); // [1, 4, 9, 16]
    println!("Evens: {:?}", evens); // [2, 4]
}
```

### 3. Ownership with Iterators
- **`iter`**: Borrow items.
- **`into_iter`**: Take ownership of items.
- **`iter_mut`**: Mutably borrow items.

```rust
fn main() {
    let mut numbers = vec![1, 2, 3];
    for num in numbers.iter_mut() {
        *num *= 2;
    }
    println!("{:?}", numbers); // [2, 4, 6]
}
```

## Key Differences from Your Languages

| Concept         | TypeScript/Python | Rust                  |
|-----------------|-------------------|-----------------------|
| Arrays          | Dynamic           | Fixed or growable     |
| Dictionaries    | `dict`/`object`  | `HashMap`             |
| Iterators       | Eager evaluation  | Lazy evaluation       |

## Your Exercise

### Task 1: Vector Operations
1. Create a vector of integers.
2. Add 5 numbers to it.
3. Remove the last number.
4. Print the sum of all remaining numbers using an iterator.

### Task 2: HashMap Operations
1. Create a `HashMap` to store names and scores.
2. Add 3 entries.
3. Update one of the scores.
4. Print all names and scores.

### Task 3: Iterator Adapters
1. Create a vector of integers.
2. Use `filter` to keep only even numbers.
3. Use `map` to square each number.
4. Collect the results into a new vector and print it.

### Success Criteria
✓ Code compiles without warnings
✓ Functions work correctly
✓ Clean, readable code
✓ Good variable names

Once you're done, show me your code and I'll review it!