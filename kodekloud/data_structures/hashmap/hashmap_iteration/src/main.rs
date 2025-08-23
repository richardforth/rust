use std::collections:: HashMap;

fn main() {
    let mut student_grades = HashMap:: new();
    student_grades.insert("Alice", 85);
    student_grades.insert("Bob", 78);

    // Iterating through the HashMap
    for (student, grade) in &student_grades {
        println!("{}: {}", student, grade);
    }
}
