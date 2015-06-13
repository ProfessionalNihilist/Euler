fn main() {
  let mut multiples: Vec<i32> = vec![];
 
  for x in 1..1000 {
    if x % 5 == 0 || x % 3 == 0 {
      multiples.push(x);
    }
  }
 
  let sum = multiples.iter().fold(0, |acc, item| acc + item);
 
  println!("Sum is {}", sum);
}
