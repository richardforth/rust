use std::collections:: HashMap;

fn main() {
	let mut student_grades = HashMap::new();
    student_grades.insert("Alice", 85);
	student_grades.remove("Alice");

	println!("{:?}", student_grades); // Empty HashMap
}
