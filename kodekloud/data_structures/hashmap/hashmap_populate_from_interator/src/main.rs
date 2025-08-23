use std::collections:: HashMap;

fn main() {
    // Vectors of names and grades
    let names = vec! ["Alice", "Bob", "Charlie"];
    let grades = vec! [85, 78, 92];

	// Creating a HashMap using the collect() method
    let student_grades: HashMap<_, _> = 
		names.into_iter()
		.zip(grades.into_iter())
		.collect();
	
    println!("{:?}", student_grades);
}
