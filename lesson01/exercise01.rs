fn celsius_to_fahrenheit(celsius: f64) -> f64 {
  celsius * 1.8 + 32.
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
  (fahrenheit - 32.) / 1.8
}

fn is_extreme_temp(celsius: f64) -> bool {
  celsius < -40. || celsius > 50.
}

fn main() {
  let celsius_temperature1 = 30.;
  let celsius_temperature2 = 100.;
  let celsius_temperature3 = -40.;
  let fahrenheit_temperature1 = 30.;
  let fahrenheit_temperature2 = 100.;
  let fahrenheit_temperature3 = -40.;

  let converted_to_fahrenheit1 = celsius_to_fahrenheit(celsius_temperature1);
  let converted_to_fahrenheit2 = celsius_to_fahrenheit(celsius_temperature2);
  let converted_to_fahrenheit3 = celsius_to_fahrenheit(celsius_temperature3);
  let converted_to_celsius1 = fahrenheit_to_celsius(fahrenheit_temperature1);
  let converted_to_celsius2 = fahrenheit_to_celsius(fahrenheit_temperature2);
  let converted_to_celsius3 = fahrenheit_to_celsius(fahrenheit_temperature3);

  println!("Temperature in Celsius is {} and in Fahrenheit is {}", celsius_temperature1, converted_to_fahrenheit1);
  println!("Temperature in Celsius is {} and in Fahrenheit is {}", celsius_temperature2, converted_to_fahrenheit2);
  println!("Temperature in Celsius is {} and in Fahrenheit is {}", celsius_temperature3, converted_to_fahrenheit3);
  println!("Temperature in Fahrenheit is {} and in Celsius is {}", fahrenheit_temperature1, converted_to_celsius1);
  println!("Temperature in Fahrenheit is {} and in Celsius is {}", fahrenheit_temperature2, converted_to_celsius2);
  println!("Temperature in Fahrenheit is {} and in Celsius is {}", fahrenheit_temperature3, converted_to_celsius3);

  let is_temperature1_extreme = is_extreme_temp(celsius_temperature1);
  let is_temperature2_extreme = is_extreme_temp(celsius_temperature2);
  let is_temperature3_extreme = is_extreme_temp(celsius_temperature3);

  println!("The Temperature is {} and is {} Extreme", celsius_temperature1, if is_temperature1_extreme {"not"} else {""} );
  println!("The Temperature is {} and is {} Extreme", celsius_temperature2, if is_temperature2_extreme {"not"} else {""} );
  println!("The Temperature is {} and is {} Extreme", celsius_temperature3, if is_temperature3_extreme {"not"} else {""} );
}