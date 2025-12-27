# Code Review for Lesson 6

## Task 1: Define and Implement a Trait
### Code:
```rust
trait Describable {
    fn describe(&self) -> String {
        String::from("(No description available)")
    }
}

struct Person {
    name: String,
    age: i8,
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("Person Name: {}\nPerson Age: {}", self.name, self.age)
    }
}

struct Car {
    make: String,
    model: String,
}

impl Describable for Car {
    fn describe(&self) -> String {
        format!("Car Make: {}\n Car Model: {}", self.make, self.model)
    }
}
```

### Feedback:
1. **Correctness**:
   - The `Describable` trait is correctly defined with a default implementation.
   - The `Person` and `Car` structs implement the `Describable` trait correctly, overriding the default implementation.

2. **Readability**:
   - The code is clean and easy to follow.
   - The use of `format!` for string formatting is appropriate.

3. **Improvement**:
   - None! This implementation is perfect.

---

## Task 2: Default Implementation
### Code:
```rust
struct Cat {}

impl Describable for Cat {}
```

### Feedback:
1. **Correctness**:
   - The `Cat` struct uses the default implementation of the `Describable` trait correctly.

2. **Readability**:
   - The code is concise and demonstrates the use of default implementations effectively.

3. **Improvement**:
   - None! This implementation is perfect.

---

## Task 3: Trait Bounds
### Code:
```rust
fn print_description<T: Describable>(item: &T) -> () {
    println!("{}", item.describe());
}
```

### Feedback:
1. **Correctness**:
   - The `print_description` function correctly uses a trait bound to ensure that the generic type `T` implements the `Describable` trait.
   - The function works as expected with all types that implement `Describable`.

2. **Readability**:
   - The code is clean and demonstrates the use of trait bounds effectively.

3. **Improvement**:
   - None! This implementation is perfect.

---

## Main Function
### Code:
```rust
fn main() {
    let person = Person {
        name: String::from("Mohammad-Hossein"),
        age: 24,
    };
    let car = Car {
        make: String::from("Benz"),
        model: String::from("230 W115"),
    };
    let cat = Cat {};

    print_description(&person);
    print_description(&car);
    print_description(&cat);
}
```

### Feedback:
1. **Correctness**:
   - The `main` function demonstrates all tasks correctly.
   - The `print_description` function is called for `Person`, `Car`, and `Cat`, and the output is as expected.

2. **Readability**:
   - The code is well-structured and easy to follow.

3. **Improvement**:
   - None! This implementation is perfect.

---

## Overall Suggestions
1. **Edge Cases**:
   - Test additional edge cases, such as:
     - Empty strings for `name`, `make`, or `model`.
     - Negative or very large values for `age`.

2. **Consistency**:
   - Ensure consistent formatting in the `describe` method for all structs.

3. **Scalability**:
   - Consider adding more structs that implement the `Describable` trait to test scalability.