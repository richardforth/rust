fn main() {
  let a = 10;
  let b = 60;
  let c:f64;
  let d:f64;
  c = 7.89;
  d = -4.42;
  
  println!("\na = {} b = {} c = {}, d = {}",a,b,c,d);
  println!("\n\na > b: {}", (a > b)); 
  println!("\nd < c: {}", (d < c));
  println!("\na == b: {}", (a == b)); 
  println!("\nc != d: {}", (c != d)); 
  println!("\na <= b: {}", (a <= b)); 
  println!("\nd >= c: {}", (d >= c)); 
}
