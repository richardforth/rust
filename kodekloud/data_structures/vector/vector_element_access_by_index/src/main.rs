fn main() {
	let v = vec![1, 2, 3];
	let third: &i32 = &v[2];
	println!("The third element is {}", third);
}
