fn main() {
	let mut v = vec![1, 2, 3];
	println!("v: {:?}", v);

	// len()
	println!("Length of the vector: {}", v.len());

	// is_empty()
	println!("Is the vector empty? {}", v.is_empty());

	// contains(&reference)
	println!("Does the vector contain 2? {}", v.contains(&2));

	// Notes on contains() vs remove()
	// contains uses references to values to avoid ownership issues: &2 → "compare values" (does vector contain the integer 2)
	// remove uses indexes: 2 → "position in the vector"

	// remove(index_no)
	let removed = v.remove(2); //returns a value at the index, and assigned to "removed" variable

	println!("Removed element at index 2: {}", removed);
	println!("Vector after removal: {:?}", v);
	
	// extend()
	v.extend(vec![4, 5, 6]);
	println!("Vector v after extending: {:?}", v);

	
}
