fn main () {
  // This is a hands-on lab from the book, exploring algebra formulas
  // Credit goes to the author, not me, however I did add some additional
  // Inline comments for my own benefit, and maybe styled it myself.
  let a:isize = 5;
  let b:isize = 8;
  let a_p_b_2:isize;
  let a_m_b_2:isize;
  let a_p_b_3:isize;
  let a_m_b_3:isize;
  let a_2_m_b_2:isize;

  // Algebra expressions
  a_p_b_2 = (a * a) + (2 * a * b) + (b * b);
  a_m_b_2 = (a * a) - (2 * a * b) + (b * b);
  a_2_m_b_2 = (a + b) * (a - b);
  a_p_b_3 = (a * a * a) + ((3 * a * b) * (a + b)) + (b * b * b);
  a_m_b_3 = (a * a * a) - ((3 * a* b) * (a - b)) - (b * b * b);
 
  // Print everything
  println!("\na = {} b = {}", a, b); 
  println!("\n(a + b)^2 = {}", a_p_b_2); 
  println!("\n(a - b)^2 = {}", a_m_b_2); 
  println!("\na^2 - b^2 = {}", a_2_m_b_2); 
  println!("\n(a + b)^3 = {}", a_p_b_3); 
  println!("\n(a - b)^3 = {}", a_m_b_3); 
}
