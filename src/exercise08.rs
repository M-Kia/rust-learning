fn smallest <T: PartialOrd>(list: &[T]) -> &T {
  let mut smallest_number = &list[0];
  for item in list {
    if item < smallest_number {
      smallest_number = item
    }
  }
  smallest_number
}

struct Kilometers(f64);

fn get_miles(length: Kilometers) -> f64 {
  length * 0.6214
}

type Callback = Box<dyn Fn() + Send + 'static>;

fn count_and_run(count: u8, callback: Callback) -> () {
  for i in 0..count{
    println!("Number {}", i);
  }
  callback();
}

fn main(){
  // Task 1
  println!("{}", smallest(&[2, 12, 32, 43,52, 34, 26, 10, 4, 1]));
  println!("{}", smallest(&[1.3, 0.5, 19.2, 32.4, 0.7, 12.42]));

  // Task 2
  let length = Kilometers(120.);
  println!("{} Kilometers is {} in Miles", length.0, get_miles(length));

  // Task 3
  count_and_run(10, |_| -> println!("Hello World!"));
}