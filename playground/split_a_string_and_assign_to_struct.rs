struct Person {
    first_name: String,
    last_name: String,
    age: u32,
}

fn main() {
    let input = "Alice Smith 30";
    let mut parts = input.split_whitespace();

    let person = Person {
        first_name: parts.next().unwrap().to_string(),
        last_name: parts.next().unwrap().to_string(),
        age: parts.next().unwrap().parse().unwrap(),
    };

    println!("{} {} is {} years old", person.first_name, person.last_name, person.age);
}
