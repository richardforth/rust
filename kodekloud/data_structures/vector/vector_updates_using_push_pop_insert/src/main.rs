fn main() {
	let mut v = vec![1,2,3];
	println!("v: {:?}", v);
	
        // First, lets push 4 onto the vector: Result: [1, 2, 3, 4]
	v.push(4);
	println!("v.push(4): {:?}", v);

	// Now let's pop a value off the vector: Result: [1, 2, 3]
	// LIFO : Last In First Out
	v.pop();
	println!("v.pop(): {:?}", v);

	// Now lets put 4 back, but using insert, we specify the index:  Result: [1, 2, 3, 4]
	v.insert(3, 4);
	println!("v.insert(3, 4): {:?}", v);

	// Lets also add 0 at index 0 to show we can insert anywhere: Result: [0, 1, 2, 3, 4]
	v.insert(0, 0);
	println!("v.insert(0, 0): {:?}", v);
}
