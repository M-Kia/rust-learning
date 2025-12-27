# Lesson 11: Concurrency

## Key Concepts

### 1. Threads
Rust provides the `std::thread` module for creating threads.

#### Example:
```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    handle.join().unwrap();
}
```

### 2. Channels
Channels allow threads to communicate.

#### Example:
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("Hello from the thread!").unwrap();
    });

    println!("Received: {}", rx.recv().unwrap());
}
```

### 3. Mutex
A `Mutex` ensures safe access to shared data.

#### Example:
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    for _ in 0..10 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut num = data.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *data.lock().unwrap());
}
```

## Your Exercise

### Task 1: Threads
1. Create a program that spawns 5 threads.
2. Each thread should print a message.

### Task 2: Channels
1. Create a program that spawns a thread to send messages through a channel.
2. The main thread should receive and print these messages.

### Task 3: Mutex
1. Create a program that uses a `Mutex` to safely increment a counter from multiple threads.

### Success Criteria
✓ Code compiles without warnings
✓ Functions work correctly
✓ Clean, readable code
✓ Good variable names