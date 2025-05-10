fn main() {

  // Declare some variables
  // Note the use of 'mut' to make them mutable
  // as rust variables are immutable by default for memory safety
  let mut a:isize = -54;
  let mut b:isize = 73;
  let mut x:f64 =  -3.87;
  let mut y:f64 =  4.29;

  // Print everything
  println!("=================================================");
  println!("a = {} b = {} x = {} y = {}",a,b,x,y);
  println!("=================================================");

  a += b; // Compound Addition Operator
  println!("\na = {} after performing a += b", a);

  b -= a; // Compound Subtraction Operator
  println!("\nb = {} after performing b -= a", b);

  x *= y; // Compound Multiplication Operator
  println!("\nx = {} after performing x *= y", x);

  y /= x; // Compound Division Operator
  println!("\ny = {} after performing y /= x", y);

  a %= b; // Compound Modulo Operator (remainder of division)
  println!("\na = {} after performing a %= b", a);

  a |= b; // Compound Bitwise Logical OR Operator
  println!("\na = {} after performing a |= b", a);
  
  a ^= b; // Compound Bitwise Logical XOR Operator
  println!("\na = {} after performing a ^= b", a);

  a <<= 3; // Compound Bitwise Left Shift Operator
  println!("\na = {} after performing a <<= 3", a);

  a &= b; // Compound Bitwise Logical AND Operator
  println!("\na = {} after performing a &= b", a);

  b >>= 2; // Compound Bitwise Right Shift Operator
  println!("\nb = {} after performing b >>= 2", b);

}
