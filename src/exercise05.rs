use std::collections::HashMap;

fn main() {
    // Task 1
    let mut numbers = vec![];
    numbers.push(2);
    numbers.push(7);
    numbers.push(8);
    numbers.push(11);
    numbers.push(13);
    numbers.pop();
    let sum: i32 = numbers.iter().sum();
    println!("Sum of {:?} is {}", numbers, sum);

    // Task 2
    let mut scores = HashMap::new();
    scores.insert("Mohammad-Hossein", 16);
    scores.insert("Amir-Hossein", 17);
    scores.insert("Sadegh", 18);
    scores.insert("Mohammad-Hossein", 20);

    for (name, score) in &scores {
        println!("{}'s score is {}", name, score);
    }
    
    // Task 3
    let numbers: Vec<i32> = vec![2, 5, 7, 8, 11, 26];
    let changed_numbers: Vec<i32> = numbers.into_iter().filter(|x| x % 2 == 0).map(|x| x.pow(2)).collect();

    println!("New numbers are {:?}", changed_numbers);
}
