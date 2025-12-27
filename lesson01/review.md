# Code Review for Lesson 1

## Strengths:
1. **Correct Functionality**:
   - `celsius_to_fahrenheit` and `fahrenheit_to_celsius` are implemented correctly using the right formulas.
   - `is_extreme_temp` correctly identifies extreme temperatures.

2. **Readable Code**:
   - Clear variable names like `celsius_temperature1` make the code easy to follow.
   - Proper use of `println!` for output.

3. **Bonus Challenge**:
   - You implemented `is_extreme_temp` and used it in `main()`—great job!

## Improvements:
1. **Output Formatting**:
   - The extreme temperature check output is slightly unclear. For example:
     ```rust
     println!("The Temperature is {} and is {} Extreme", celsius_temperature1, if is_temperature1_extreme {"not"} else {""} );
     ```
     This could be simplified for better readability.

2. **Code Duplication**:
   - The repetitive blocks for temperature conversion and extreme checks can be refactored into loops or functions to reduce redundancy.

3. **Edge Cases**:
   - You could add edge cases like `0°C` or `32°F` to test the boundaries.

4. **Clarity in Extreme Check**:
   - The `if` condition for "not extreme" is a bit confusing. It can be rephrased for clarity.

## Suggested Refactor:
Here’s a cleaner version of your `main()` function:

```rust
fn main() {
    let celsius_temperatures = [30., 100., -40.];
    let fahrenheit_temperatures = [30., 100., -40.];

    for &celsius in &celsius_temperatures {
        let fahrenheit = celsius_to_fahrenheit(celsius);
        println!("Temperature in Celsius: {} -> Fahrenheit: {}", celsius, fahrenheit);
        println!(
            "Is {}°C extreme? {}",
            celsius,
            if is_extreme_temp(celsius) { "Yes" } else { "No" }
        );
    }

    for &fahrenheit in &fahrenheit_temperatures {
        let celsius = fahrenheit_to_celsius(fahrenheit);
        println!("Temperature in Fahrenheit: {} -> Celsius: {}", fahrenheit, celsius);
    }
}
```

### Why This Refactor?
1. **Reduced Repetition**:
   - Instead of repeating the same logic for each temperature, we use loops.

2. **Improved Readability**:
   - The extreme temperature check is clearer and more concise.

3. **Scalability**:
   - Adding more temperatures to test is as simple as appending to the arrays.