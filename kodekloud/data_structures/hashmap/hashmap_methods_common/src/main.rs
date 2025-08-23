use std::collections:: HashMap;

fn main() {
	let mut student_grades = HashMap::new();
	student_grades.insert("Alice", 85);
	student_grades.insert("Bob", 78);
	student_grades.insert("George", 92);
	student_grades.insert("Sally", 65);

	println!("{:?}", student_grades);
	
	// len()
	println!("Length of HashMap is {}", student_grades.len());
	
	// is_empty()
	println!("Is HashMap empty? {}", student_grades.is_empty());
	
	// insert()
	student_grades.insert("Kenny", 89);
	println!("{:?}", student_grades);
	
	// remove()
	// "You Killed Kenny!" - Cartman
	student_grades.remove("Kenny");
	println!("{:?}", student_grades);

	// contains()
	println!("Does HashMap contain key Kenny? {}", student_grades.contains_key("Kenny"));
	// note: there is no sister method contains_value, which is a shame

	// entry(key).or_insert(value)
	let sally_grade = student_grades.entry("Sally").or_insert(78); // should return Sally's grade as she exists in the HashMap, grade unchanged at 65
	println!("Sally's grade is {}", sally_grade);
	let kenny_grade = student_grades.entry("Kenny").or_insert(91); // Add Kenny back in with a higher grade than before
	println!("Kenny's grade is {}", kenny_grade);

}
