use std::collections:: HashMap;

fn main() {
	let mut student_grades = HashMap::new();
    student_grades.insert("Alice", 85);

	// Accessing using get() and match to avoid errors
	let grade = student_grades.get("Alice");

	match grade {
		Some(g) => println!("Alice's grade is {} // student_grades.get(\"Alice\")", g),
		None => println!("No grade found for Alice."),
	}

	// Accessing through indexing
	// Let's add a few more grades
	student_grades.insert("Bob", 78);
	student_grades.insert("George", 92);
	student_grades.insert("Sally", 65);

	// Using indexing is more straightforwad, but can result in unhandled errors (panic) if keys do not exist
	println!("Alice's grade is {} // student_grades[\"Alice\"]", student_grades["Alice"]);
	println!("Bob's grade is {} // student_grades[\"Bob\"]", student_grades["Bob"]);
	println!("George's grade is {} // student_grades[\"George\"]", student_grades["George"]);
	println!("Sally's grade is {} // student_grades[\"Sally\"]", student_grades["Sally"]);
	// println!("Kenny's grade is {} // student_grades[\"Kenny\"]", student_grades["Kenny"]); // panic if uncommented
}
