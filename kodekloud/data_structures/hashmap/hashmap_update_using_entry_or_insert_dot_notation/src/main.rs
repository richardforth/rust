use std::collections:: HashMap;

fn main() {
	let mut student_grades = HashMap::new();
	student_grades.entry("Alice").or_insert(85);
	student_grades.entry("Alice").or_insert(90); // Does not overwrite, Alice's grade of 85 is unchanged
	// if it exists, entry() returns the value
	// otherwise it inserts a new key/pair
	
	println!("{:?}", student_grades);

}
