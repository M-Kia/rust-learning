# Lesson 6: Traits

## Key Concepts

### 1. What Are Traits?
Traits in Rust are similar to interfaces in other languages. They define shared behavior that types can implement.

#### Defining a Trait:
```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

#### Implementing a Trait:
```rust
struct Article {
    headline: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}...", self.headline)
    }
}
```

#### Using a Trait:
```rust
fn main() {
    let article = Article {
        headline: String::from("Breaking News"),
        content: String::from("Something important happened!"),
    };

    println!("Summary: {}", article.summarize());
}
```

### 2. Default Implementations
Traits can provide default implementations for methods.
```rust
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

### 3. Trait Bounds
Trait bounds allow you to specify that a generic type must implement a specific trait. This ensures that the generic type has the required behavior.

#### Example:
```rust
fn notify<T: Summary>(item: &T) {
    println!("Breaking news: {}", item.summarize());
}
```
- Here, `T` is a generic type, and `T: Summary` means that `T` must implement the `Summary` trait.
- This ensures that the `notify` function can call the `summarize` method on `item`.

#### Multiple Trait Bounds:
You can specify multiple traits using `+`:
```rust
fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news: {}", item.summarize());
}
```
- This means `T` must implement both `Summary` and `Display` traits.

#### Where Clause:
For better readability, use a `where` clause for complex bounds:
```rust
fn notify<T>(item: &T)
where
    T: Summary + Display,
{
    println!("Breaking news: {}", item.summarize());
}
```

---

### 4. Derivable Traits
Derivable traits are traits that the compiler can automatically implement for your types. Common derivable traits include:
- `Debug`: Allows you to print the type using `{:?}`.
- `Clone`: Allows you to create a copy of the value.
- `PartialEq`: Allows you to compare values for equality.

#### Example:
```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point1 = Point { x: 5, y: 10 };
    let point2 = point1.clone();

    println!("{:?}", point1); // Prints: Point { x: 5, y: 10 }
    println!("Are the points equal? {}", point1 == point2); // true
}
```
- The `#[derive(...)]` attribute tells the compiler to generate implementations for the specified traits.
- This saves you from writing repetitive code.

---

### 5. Dynamic Dispatch
Dynamic dispatch allows you to call methods on trait objects at runtime. This is useful when you want to work with different types that implement the same trait.

#### Example:
```rust
fn print_summary(item: &dyn Summary) {
    println!("Summary: {}", item.summarize());
}
```
- `&dyn Summary` is a trait object. It allows you to pass any type that implements the `Summary` trait.
- The method call `item.summarize()` is resolved at runtime.

#### Why Use Dynamic Dispatch?
- Use dynamic dispatch when you need flexibility to work with multiple types at runtime.
- However, dynamic dispatch has a slight performance cost compared to static dispatch.

#### Example with Multiple Types:
```rust
struct Article {
    headline: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}...", self.headline)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let article = Article {
        headline: String::from("Breaking News"),
        content: String::from("Something important happened!"),
    };

    let tweet = Tweet {
        username: String::from("user123"),
        content: String::from("Hello, world!"),
    };

    print_summary(&article);
    print_summary(&tweet);
}
```
- Both `Article` and `Tweet` implement the `Summary` trait.
- The `print_summary` function can accept both types because it uses a trait object (`&dyn Summary`).

#### Note:
The tasks in this lesson will help you practice both **Trait Bounds** and **Dynamic Dispatch**. You will:
- Use **Trait Bounds** to create generic functions that work with any type implementing a specific trait.
- Use **Dynamic Dispatch** to handle multiple types at runtime through trait objects.

#### Difference Between Trait Bounds and Dynamic Dispatch
- **Trait Bounds**:
  - Used for **static dispatch**.
  - The compiler knows the exact type at compile time.
  - Method calls are resolved at compile time, making it faster.
  - Example:
    ```rust
    fn notify<T: Summary>(item: &T) {
        println!("Breaking news: {}", item.summarize());
    }
    ```

- **Dynamic Dispatch**:
  - Used when the exact type is not known until runtime.
  - Method calls are resolved at runtime, introducing a slight performance overhead.
  - Example:
    ```rust
    fn print_summary(item: &dyn Summary) {
        println!("Summary: {}", item.summarize());
    }
    ```

- **Key Difference**:
  - Use **trait bounds** when you want performance and the type is known at compile time.
  - Use **dynamic dispatch** when you need flexibility to work with multiple types at runtime.

## Your Exercise

### Task 1: Define and Implement a Trait
1. Define a trait `Describable` with a method `describe(&self) -> String`.
2. Implement this trait for two structs:
   - `Person` with fields `name` and `age`.
   - `Car` with fields `make` and `model`.
3. In `main()`, create instances of `Person` and `Car` and call `describe` on them.

### Task 2: Default Implementation
1. Add a default implementation to `Describable` that returns `"(No description available)"`.
2. Override this default implementation for `Person` and `Car`.

### Task 3: Trait Bounds
1. Write a function `print_description<T: Describable>(item: &T)` that prints the description of any `Describable` item.
2. Test this function with both `Person` and `Car`.

### Success Criteria
✓ Code compiles without warnings
✓ Functions work correctly
✓ Clean, readable code
✓ Good variable names