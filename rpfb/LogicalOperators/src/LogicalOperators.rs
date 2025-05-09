fn main() {
  let x:isize = 23;
  let y:isize = 44;
  const Z:isize= 23;

  // Print results of random comparisons
  println!("\nx = {} y = {} Z = {}\n",x,y,Z);  
  println!("\n(x == y): {}",(x == y));  
  println!("\n(x == Z): {}",(x == Z));  
  println!("\n(x < y): {}",(x < y));  
  println!("\n(Z > x): {}",(Z > x));
  // Combine more than one boolean expression using logical operators 
  println!("\n\n(x == y) || (Z != y): {}", ((x == y) || (Z != y)));  
  println!("\n(x == Z) && (x > y): {}", ((x == Z) && (x > y)));  
  println!("\n!(Z > x): {} ", !(Z > x));  
}
