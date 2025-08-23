fn main() {
	let v = vec![1, 2, 3];
	match v.get(2) {
		Some(third) => println!("The third element is {}", third),
		None => println!("There is no third element."),
    }
}
