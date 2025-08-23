use std::collections:: HashMap;

fn main() {
	let mut student_grades = HashMap::new();
    student_grades.insert("Alice", 85);
	student_grades.insert("Alice", 95); // overwrites the previous value

	println!("{:?}", student_grades);

}
