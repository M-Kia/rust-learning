# Lesson 12: Async/Await

## Key Concepts

### 1. Async Functions
Async functions allow you to write asynchronous code that looks synchronous.

#### Example:
```rust
async fn hello() {
    println!("Hello, world!");
}
```

### 2. Awaiting Futures
Use `.await` to wait for an async function to complete.

#### Example:
```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    sleep(Duration::from_secs(1)).await;
    println!("1 second later");
}
```

### 3. Spawning Tasks
Use `tokio::spawn` to run tasks concurrently.

#### Example:
```rust
use tokio::task;

#[tokio::main]
async fn main() {
    let handle = task::spawn(async {
        println!("Hello from a task!");
    });

    handle.await.unwrap();
}
```

## Your Exercise

### Task 1: Async Functions
1. Write an async function `fetch_data()` that simulates fetching data with a delay.
2. Call this function in `main()`.

### Task 2: Concurrent Tasks
1. Write a program that spawns 3 async tasks.
2. Each task should print a message after a delay.

### Task 3: Using Tokio
1. Add the `tokio` crate to your project.
2. Write a program that uses `tokio::time::sleep` to simulate delays.

### Success Criteria
✓ Code compiles without warnings
✓ Functions work correctly
✓ Clean, readable code
✓ Good variable names