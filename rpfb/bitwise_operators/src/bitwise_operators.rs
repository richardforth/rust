fn main() {
  let x:isize = 5;
  let y:isize = 11;

  // Print results of bitwise operations
  println!("\nx = {} y = {}\n", x, y);
  println!("\n(x | y): {}", (x | y));
  println!("\n(x & y): {}", (x & y));
  println!("\n(x ^ y): {}", (x ^ y));
  println!("\n(x << 3): {}", (x << 3));
  println!("\n(y >> 1): {}", (y >> 1));
}
