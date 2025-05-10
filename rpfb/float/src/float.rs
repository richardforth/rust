fn main() {
  // Doctor Evil Laugh
  // One Million can be expressed as 1000000 or as 1_000_000 in rust
  let one_mill: f32 = 1_000_000.00;
  let one_thou: f32 = 1_000.00;
  let total: f32 = one_mill + one_thou;
  println!("Total of {} plus {} equals {}", one_mill, one_thou, total);
}
