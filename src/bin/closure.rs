fn main() {
  let add = |a: i32, b: i32| -> i32 {
    return a + b;
  };

  let sum = add(1, 2);
  println!("sum: {}", sum);
}