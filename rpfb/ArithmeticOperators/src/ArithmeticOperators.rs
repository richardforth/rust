fn main() {
  // This is a hands-on lab from the book, exploring arithmetic operators
  // Credit goes to the author, not me, however I did add some additional
  // Inline comments for my own benefit, and maybe styled it myself.
  let a:isize = 70; // Architecture-specific unigned integer
  let b:isize = 30; // Architecture-specific unigned integer
  let x:f64 = -5.8; // float
  let y:f64 = -3.2; // float
  let sum:isize; // I assumed variable not defined without 'mut' are immutable,
  let diff:isize; // but I suspect they become immutable once values are assigned.
  let prod:f64; // also immutable
  let modulus:isize; // same here
  let quo:f64; // and here

  // perform arithmetic operations
  sum = a + b;
  diff = a -b;
  modulus = a % b;
  prod = x * y;
  quo = x / y;

  // Print everything
  println!("\na = {}, b = {}\n\nx = {}, y = {}",a,b,x,y);
  println!("\na + b = {}\na - b = {}\na % b = {}",sum,diff,modulus);
  println!("\nx * y = {}\nx / y = {}",prod,quo);
}
