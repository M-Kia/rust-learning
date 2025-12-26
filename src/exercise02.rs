fn first_word(sentence: &String) -> &str {
  let mut words = sentence.split(" ");
  let first_word = words.nth(0);
  match first_word {
    Some(word) => word,
    None => "",
  }
}

fn takes_ownership(s: String) {
  let _s1 = s;
}

fn makes_copy(n: i32) -> i32 { n }

fn append_world(s: &mut String) {
  s.push_str(" world");
}

fn main() {
  // Task 1
  let sentence = String::from("Hello to all");
  let word = first_word(&sentence);
  
  println!("The first word of sentence is: {}", word);
  
  // Task 2
  let name = String::from("Mohammad-Hossein");
  takes_ownership(name);
  let age: i32 = 24;
  makes_copy(age);

  // Task 3
  let mut temp = String::from("Hello ");
  append_world(&mut temp);
  println!("The new sentence is: {}", temp);


}