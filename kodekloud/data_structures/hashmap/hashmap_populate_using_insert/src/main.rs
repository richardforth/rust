use std::collections:: HashMap;

fn main() {
	let mut student_grades = HashMap::new();

	student_grades.insert("Alice", 85);
	student_grades.insert("Bob", 78);
	student_grades.insert("Alice", 95); // Updates existing value if present, as keys are UNIQUE

	println!("{:?}", student_grades);
}

