fn strings() {
  let s:&'static str = "Hello, world!"; // &str = string slice

  for c in s.chars() {
      println!("{}", c);
  }

  // reverse order
  for c in s.chars().rev() {
      println!("{}", c);
  }

  // third letter
  if let Some(third_char) = s.chars().nth(3) {
      println!("Third letter is '{}'", third_char);
  }

  // There is also a String type
  // It is heap allocated and valid utf-8 sequence
  // You cannot grow a &str string type. For this
  // you would use String type:
  let mut letters = String::new();
  let mut a = 'a' as u8;
  while a <= ('z' as u8) {
      letters.push(a as char);
      letters.push_str(",");
      a += 1;
  }
  println!("{}", letters);

  // &str <> String
  let u:&str = &letters;
  println!("{}", u);

  // Concatenation
  // String + str
  let v = String::from(&letters) + "abc";
  println!("{}", v);
  let w = String::from(&letters) + &letters;
  println!("{}", w);

  let mut abc = String::from("Hello world again!");
  let mut abcd = "Hello world again!".to_string();
  abc.remove(0);
  abc.push_str("!!!");
  println!("{}", abc.replace("world", "universe"));
}

fn main() {
    strings()
}
