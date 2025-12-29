fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    return x;
  } else {
    return y;
  }
}

struct BorrowedString<'a> {
  text: &'a str
}

fn first_char(s: &str)-> &str {
  s.get(..1).unwrap_or("")
}

fn main(){
  let sentences = [
    "Lorem Ipsum with 5 words.",
    "Some sentences with many words in it.",
    "Some meaningless words next to each others",
    "Lorem Ipsum calls me to do the job",
    "Lorem Ipsum works fine"
  ];

  // Task 1
  println!("{}", longest(sentences[0], sentences[3]));
  println!("{}", longest(sentences[3], sentences[4]));
  println!("{}", longest(sentences[1], sentences[2]));

  // Task 2
  let temp_sentence = BorrowedString {text: sentences[0]};
  
  println!("{}", temp_sentence.text);

  // Task 3
  println!("{}", first_char(sentences[3]));
  println!("{}", first_char(sentences[2]));
}